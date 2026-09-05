use neurite_control::{
    admit, AdmissionDenial, AdmissionPolicy, CapabilityEnvelope, IntentEnvelope,
};
use neurite_core::{CommandId, EntityId, IdNamespace, IntentId, PrincipalId, Vec2};
use neurite_events::Intent;
use neurite_protocol::{
    ActionId, CapabilityId, PartitionKey, ProtocolBinding, ProtocolId, ProtocolVersion, ScenarioId,
};

const NAMESPACE: IdNamespace = IdNamespace::new(0xC017_7010);

fn protocol() -> ProtocolBinding {
    ProtocolBinding::new(
        ProtocolId::new("hq.protocol.intent-v1").unwrap(),
        ProtocolVersion::new(1).unwrap(),
        ScenarioId::new("mobile.on-the-go").unwrap(),
    )
}

fn move_intent(actor: PrincipalId, partition: PartitionKey) -> IntentEnvelope {
    IntentEnvelope::new(
        IntentId::scoped(NAMESPACE, 2),
        actor,
        partition,
        protocol(),
        ActionId::new("hq.action.workspace.entity.move").unwrap(),
        Intent::MoveEntity {
            entity: EntityId::scoped(NAMESPACE, 3),
            position: Vec2::new(4.0, 5.0),
        },
    )
}

fn admission_policy() -> AdmissionPolicy {
    let mut policy = AdmissionPolicy::new();
    policy.require(
        ActionId::new("hq.action.workspace.entity.move").unwrap(),
        CapabilityId::new("workspace.entity.move").unwrap(),
    );
    policy
}

#[test]
fn missing_capability_is_denied_by_default() {
    let actor = PrincipalId::scoped(NAMESPACE, 1);
    let partition = PartitionKey::new("mobile.momo.primary").unwrap();
    let intent = move_intent(actor, partition.clone());
    let capabilities = CapabilityEnvelope::new(actor, partition);

    let result = admit(
        CommandId::scoped(NAMESPACE, 4),
        intent,
        &capabilities,
        &admission_policy(),
    );

    assert_eq!(
        result,
        Err(AdmissionDenial::MissingCapabilities(vec![
            CapabilityId::new("workspace.entity.move").unwrap()
        ]))
    );
}

#[test]
fn admitted_command_preserves_actor_partition_protocol_and_intent_identity() {
    let actor = PrincipalId::scoped(NAMESPACE, 5);
    let partition = PartitionKey::new("mobile.momo.primary").unwrap();
    let intent = move_intent(actor, partition.clone());
    let source_intent = intent.id;
    let expected_protocol = intent.protocol.clone();
    let mut capabilities = CapabilityEnvelope::new(actor, partition.clone());
    capabilities.grant(CapabilityId::new("workspace.entity.move").unwrap());

    let command = admit(
        CommandId::scoped(NAMESPACE, 6),
        intent,
        &capabilities,
        &admission_policy(),
    )
    .unwrap();

    assert_eq!(command.source_intent, source_intent);
    assert_eq!(command.actor, actor);
    assert_eq!(command.partition, partition);
    assert_eq!(command.protocol, expected_protocol);
}

#[test]
fn grants_cannot_cross_principals_or_partitions() {
    let actor = PrincipalId::scoped(NAMESPACE, 7);
    let other_actor = PrincipalId::scoped(NAMESPACE, 8);
    let mobile_partition = PartitionKey::new("mobile.momo.primary").unwrap();
    let desktop_partition = PartitionKey::new("desktop.momo.primary").unwrap();

    let actor_mismatch = admit(
        CommandId::scoped(NAMESPACE, 9),
        move_intent(actor, mobile_partition.clone()),
        &CapabilityEnvelope::new(other_actor, mobile_partition.clone()),
        &admission_policy(),
    );
    assert_eq!(actor_mismatch, Err(AdmissionDenial::PrincipalMismatch));

    let partition_mismatch = admit(
        CommandId::scoped(NAMESPACE, 10),
        move_intent(actor, mobile_partition),
        &CapabilityEnvelope::new(actor, desktop_partition),
        &admission_policy(),
    );
    assert_eq!(partition_mismatch, Err(AdmissionDenial::PartitionMismatch));
}

#[test]
fn an_unknown_action_is_denied_instead_of_inheriting_zero_requirements() {
    let actor = PrincipalId::scoped(NAMESPACE, 11);
    let partition = PartitionKey::new("mobile.momo.primary").unwrap();
    let unknown_action = ActionId::new("vendor.unregistered.action").unwrap();
    let intent = IntentEnvelope::new(
        IntentId::scoped(NAMESPACE, 12),
        actor,
        partition.clone(),
        protocol(),
        unknown_action.clone(),
        Intent::MoveEntity {
            entity: EntityId::scoped(NAMESPACE, 13),
            position: Vec2::new(1.0, 1.0),
        },
    );

    let result = admit(
        CommandId::scoped(NAMESPACE, 14),
        intent,
        &CapabilityEnvelope::new(actor, partition),
        &admission_policy(),
    );

    assert_eq!(result, Err(AdmissionDenial::UnknownAction(unknown_action)));
}
