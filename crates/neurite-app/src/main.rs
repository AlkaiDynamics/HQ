#![forbid(unsafe_code)]

use neurite_core::{EntityId, IdNamespace, SpatialState};
use neurite_scene::SpatialEntity;
use neurite_workspace::Workspace;

fn main() {
    let mut workspace = Workspace::create("local-default");
    let entity = SpatialEntity::new(
        EntityId::scoped(IdNamespace::new(1), 1),
        Some(SpatialState::default()),
    );
    let entity_id = workspace
        .scene
        .add_entity(entity)
        .expect("new entity id must be unique");
    workspace.set_note(entity_id, "HQ native kernel is alive.");

    println!(
        "workspace={} entities={} dirty={}",
        workspace.id,
        workspace.scene.entities().count(),
        workspace.is_dirty()
    );
}
