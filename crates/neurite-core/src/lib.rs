#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new() -> Self {
        Self(next_id())
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationshipId(pub u64);

impl RelationshipId {
    pub fn new() -> Self {
        Self(next_id())
    }
}

impl Default for RelationshipId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionId(pub u64);

impl ProjectionId {
    pub fn new() -> Self {
        Self(next_id())
    }
}

impl Default for ProjectionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialState {
    pub position: Vec2,
    pub scale: f64,
    pub anchored: bool,
}

impl Default for SpatialState {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            scale: 1.0,
            anchored: false,
        }
    }
}
