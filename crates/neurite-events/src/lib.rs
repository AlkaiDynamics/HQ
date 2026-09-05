#![forbid(unsafe_code)]

use neurite_core::{EntityId, RelationshipId, SpatialState, Vec2};
use neurite_scene::{Directionality, RelationshipKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    CreateEntity { spatial: Option<SpatialState> },
    DeleteEntity { entity: EntityId },
    MoveEntity { entity: EntityId, position: Vec2 },
    ScaleEntity { entity: EntityId, scale: f64 },
    SetAnchored { entity: EntityId, anchored: bool },
    Connect {
        a: EntityId,
        b: EntityId,
        kind: RelationshipKind,
    },
    SetDirectionality {
        relationship: RelationshipId,
        directionality: Directionality,
    },
    Disconnect { relationship: RelationshipId },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    EntityCreated { entity: EntityId },
    EntityDeleted { entity: EntityId },
    EntityMoved { entity: EntityId, position: Vec2 },
    EntityScaled { entity: EntityId, scale: f64 },
    AnchorChanged { entity: EntityId, anchored: bool },
    RelationshipCreated { relationship: RelationshipId },
    RelationshipDirectionChanged {
        relationship: RelationshipId,
        directionality: Directionality,
    },
    RelationshipDeleted { relationship: RelationshipId },
    Rejected { reason: String },
}
