#![forbid(unsafe_code)]

use neurite_core::EntityId;
use neurite_scene::Scene;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Workspace {
    pub id: String,
    pub schema_version: u32,
    pub scene: Scene,
    pub notes: BTreeMap<EntityId, String>,
    dirty: bool,
}

impl Workspace {
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;

    pub fn create(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            scene: Scene::default(),
            notes: BTreeMap::new(),
            dirty: false,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    pub fn set_note(&mut self, entity: EntityId, text: impl Into<String>) {
        self.notes.insert(entity, text.into());
        self.mark_dirty();
    }

    pub fn delete_note(&mut self, entity: EntityId) -> Option<String> {
        let removed = self.notes.remove(&entity);
        if removed.is_some() {
            self.mark_dirty();
        }
        removed
    }
}
