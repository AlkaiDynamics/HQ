use neurite_core::{EntityId, IdNamespace, RelationshipId, SpatialState};
use neurite_scene::{Relationship, RelationshipKind, Scene, SpatialEntity};

#[test]
fn scene_preserves_ids_from_persistence_or_remote_clients() {
    let namespace = IdNamespace::new(0xA11D_001D);
    let entity_id = EntityId::scoped(namespace, 1);
    let mut scene = Scene::default();

    let inserted = scene
        .add_entity(SpatialEntity::new(entity_id, Some(SpatialState::default())))
        .unwrap();

    assert_eq!(inserted, entity_id);
}

#[test]
fn relationship_preserves_an_externally_assigned_id() {
    let namespace = IdNamespace::new(0xD35C_7001);
    let a = EntityId::scoped(namespace, 1);
    let b = EntityId::scoped(namespace, 2);
    let relationship_id = RelationshipId::scoped(namespace, 3);
    let mut scene = Scene::default();
    scene
        .add_entity(SpatialEntity::new(a, Some(SpatialState::default())))
        .unwrap();
    scene
        .add_entity(SpatialEntity::new(b, Some(SpatialState::default())))
        .unwrap();

    let inserted = scene
        .add_relationship(Relationship::new(
            relationship_id,
            a,
            b,
            RelationshipKind::Visual,
        ))
        .unwrap();

    assert_eq!(inserted, relationship_id);
}
