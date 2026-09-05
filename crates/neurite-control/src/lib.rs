#![forbid(unsafe_code)]

use neurite_core::{CommandId, IntentId, PrincipalId};
use neurite_events::Intent;
use neurite_protocol::{ActionId, CapabilityId, PartitionKey, ProtocolBinding};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq)]
pub struct IntentEnvelope {
    pub id: IntentId,
    pub actor: PrincipalId,
    pub partition: PartitionKey,
    pub protocol: ProtocolBinding,
    pub action: ActionId,
    pub body: Intent,
}

impl IntentEnvelope {
    pub fn new(
        id: IntentId,
        actor: PrincipalId,
        partition: PartitionKey,
        protocol: ProtocolBinding,
        action: ActionId,
        body: Intent,
    ) -> Self {
        Self {
            id,
            actor,
            partition,
            protocol,
            action,
            body,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AdmissionPolicy {
    requirements: BTreeMap<ActionId, BTreeSet<CapabilityId>>,
}

impl AdmissionPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, action: ActionId) {
        self.requirements.entry(action).or_default();
    }

    pub fn require(&mut self, action: ActionId, capability: CapabilityId) {
        self.requirements
            .entry(action)
            .or_default()
            .insert(capability);
    }

    fn requirements_for(&self, action: &ActionId) -> Option<&BTreeSet<CapabilityId>> {
        self.requirements.get(action)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityEnvelope {
    pub principal: PrincipalId,
    pub partition: PartitionKey,
    granted_capabilities: BTreeSet<CapabilityId>,
}

impl CapabilityEnvelope {
    pub fn new(principal: PrincipalId, partition: PartitionKey) -> Self {
        Self {
            principal,
            partition,
            granted_capabilities: BTreeSet::new(),
        }
    }

    pub fn grant(&mut self, capability: CapabilityId) {
        self.granted_capabilities.insert(capability);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandEnvelope {
    pub id: CommandId,
    pub source_intent: IntentId,
    pub actor: PrincipalId,
    pub partition: PartitionKey,
    pub protocol: ProtocolBinding,
    pub action: ActionId,
    pub body: Intent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionDenial {
    PrincipalMismatch,
    PartitionMismatch,
    UnknownAction(ActionId),
    MissingCapabilities(Vec<CapabilityId>),
}

pub fn admit(
    command_id: CommandId,
    intent: IntentEnvelope,
    capabilities: &CapabilityEnvelope,
    policy: &AdmissionPolicy,
) -> Result<CommandEnvelope, AdmissionDenial> {
    if intent.actor != capabilities.principal {
        return Err(AdmissionDenial::PrincipalMismatch);
    }
    if intent.partition != capabilities.partition {
        return Err(AdmissionDenial::PartitionMismatch);
    }

    let requirements = policy
        .requirements_for(&intent.action)
        .ok_or_else(|| AdmissionDenial::UnknownAction(intent.action.clone()))?;
    let missing = requirements
        .difference(&capabilities.granted_capabilities)
        .cloned()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(AdmissionDenial::MissingCapabilities(missing));
    }

    Ok(CommandEnvelope {
        id: command_id,
        source_intent: intent.id,
        actor: intent.actor,
        partition: intent.partition,
        protocol: intent.protocol,
        action: intent.action,
        body: intent.body,
    })
}
