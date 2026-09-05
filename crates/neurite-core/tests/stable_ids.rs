use neurite_core::{
    CommandId, EntityId, EventId, IdNamespace, IntentId, PrincipalId, ProjectionId, RelationshipId,
};

#[test]
fn scoped_ids_reconstruct_exactly_after_restart() {
    let namespace = IdNamespace::new(0xA11C_E001_D001_CE01);

    let first = EntityId::scoped(namespace, 42);
    let reconstructed = EntityId::from_u128(first.as_u128());

    assert_eq!(first, reconstructed);
    assert_eq!(reconstructed.namespace(), namespace);
    assert_eq!(reconstructed.local(), 42);
}

#[test]
fn separate_installation_namespaces_cannot_collide() {
    let mobile = EntityId::scoped(IdNamespace::new(10), 7);
    let desktop = EntityId::scoped(IdNamespace::new(11), 7);

    assert_ne!(mobile, desktop);
}

#[test]
fn every_persisted_identifier_uses_the_same_scoped_shape() {
    let namespace = IdNamespace::new(99);

    let relationship = RelationshipId::scoped(namespace, 2);
    let projection = ProjectionId::scoped(namespace, 3);

    assert_eq!(relationship.namespace(), namespace);
    assert_eq!(relationship.local(), 2);
    assert_eq!(projection.namespace(), namespace);
    assert_eq!(projection.local(), 3);
}

#[test]
fn control_plane_identifiers_are_durable_and_namespaced() {
    let namespace = IdNamespace::new(123);

    assert_eq!(PrincipalId::scoped(namespace, 1).namespace(), namespace);
    assert_eq!(IntentId::scoped(namespace, 2).local(), 2);
    assert_eq!(CommandId::scoped(namespace, 3).local(), 3);
    assert_eq!(EventId::scoped(namespace, 4).local(), 4);
}
