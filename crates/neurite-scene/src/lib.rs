#![forbid(unsafe_code)]

use neurite_core::{EntityId, ProjectionId, RelationshipId, SpatialState};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipKind {
    Visual,
    Semantic,
    Layout,
    Runtime,
    Authority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directionality {
    Undirected,
    Directed { from: EntityId, to: EntityId },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpatialEntity {
    pub id: EntityId,
    pub spatial: Option<SpatialState>,
    pub semantic_entity_ref: Option<String>,
    pub projections: BTreeSet<ProjectionId>,
    pub relationships: BTreeSet<RelationshipId>,
}

impl SpatialEntity {
    pub fn new(spatial: Option<SpatialState>) -> Self {
        Self {
            id: EntityId::new(),
            spatial,
            semantic_entity_ref: None,
            projections: BTreeSet::new(),
            relationships: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    pub id: RelationshipId,
    pub a: EntityId,
    pub b: EntityId,
    pub kind: RelationshipKind,
    pub directionality: Directionality,
}

impl Relationship {
    pub fn new(a: EntityId, b: EntityId, kind: RelationshipKind) -> Self {
        Self {
            id: RelationshipId::new(),
            a,
            b,
            kind,
            directionality: Directionality::Undirected,
        }
    }
}

#[derive(Debug, Default)]
pub struct Scene {
    entities: BTreeMap<EntityId, SpatialEntity>,
    relationships: BTreeMap<RelationshipId, Relationship>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneError {
    MissingEntity(EntityId),
    MissingRelationship(RelationshipId),
    DuplicateEntity(EntityId),
}

impl Scene {
    pub fn add_entity(&mut self, entity: SpatialEntity) -> Result<EntityId, SceneError> {
        let id = entity.id;
        if self.entities.insert(id, entity).is_some() {
            return Err(SceneError::DuplicateEntity(id));
        }
        Ok(id)
    }

    pub fn entity(&self, id: EntityId) -> Option<&SpatialEntity> {
        self.entities.get(&id)
    }

    pub fn entity_mut(&mut self, id: EntityId) -> Option<&mut SpatialEntity> {
        self.entities.get_mut(&id)
    }

    pub fn entities(&self) -> impl Iterator<Item = &SpatialEntity> {
        self.entities.values()
    }

    pub fn add_relationship(
        &mut self,
        relationship: Relationship,
    ) -> Result<RelationshipId, SceneError> {
        if !self.entities.contains_key(&relationship.a) {
            return Err(SceneError::MissingEntity(relationship.a));
        }
        if !self.entities.contains_key(&relationship.b) {
            return Err(SceneError::MissingEntity(relationship.b));
        }

        let id = relationship.id;
        let a = relationship.a;
        let b = relationship.b;
        self.relationships.insert(id, relationship);
        self.entities.get_mut(&a).expect("checked above").relationships.insert(id);
        self.entities.get_mut(&b).expect("checked above").relationships.insert(id);
        Ok(id)
    }

    pub fn relationship(&self, id: RelationshipId) -> Option<&Relationship> {
        self.relationships.get(&id)
    }

    pub fn relationship_mut(&mut self, id: RelationshipId) -> Option<&mut Relationship> {
        self.relationships.get_mut(&id)
    }

    pub fn relationships(&self) -> impl Iterator<Item = &Relationship> {
        self.relationships.values()
    }

    pub fn remove_relationship(&mut self, id: RelationshipId) -> Result<Relationship, SceneError> {
        let relationship = self
            .relationships
            .remove(&id)
            .ok_or(SceneError::MissingRelationship(id))?;
        if let Some(entity) = self.entities.get_mut(&relationship.a) {
            entity.relationships.remove(&id);
        }
        if let Some(entity) = self.entities.get_mut(&relationship.b) {
            entity.relationships.remove(&id);
        }
        Ok(relationship)
    }

    pub fn remove_entity(&mut self, id: EntityId) -> Result<SpatialEntity, SceneError> {
        let relationship_ids = self
            .entities
            .get(&id)
            .ok_or(SceneError::MissingEntity(id))?
            .relationships
            .iter()
            .copied()
            .collect::<Vec<_>>();

        for relationship_id in relationship_ids {
            self.remove_relationship(relationship_id)?;
        }

        self.entities.remove(&id).ok_or(SceneError::MissingEntity(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deleting_entity_removes_incident_relationships_but_not_other_entity() {
        let mut scene = Scene::default();
        let a = SpatialEntity::new(Some(SpatialState::default()));
        let b = SpatialEntity::new(Some(SpatialState::default()));
        let a_id = scene.add_entity(a).unwrap();
        let b_id = scene.add_entity(b).unwrap();
        let rel_id = scene
            .add_relationship(Relationship::new(a_id, b_id, RelationshipKind::Visual))
            .unwrap();

        scene.remove_entity(a_id).unwrap();

        assert!(scene.entity(a_id).is_none());
        assert!(scene.entity(b_id).is_some());
        assert!(scene.relationship(rel_id).is_none());
    }

    #[test]
    fn semantic_and_visual_relationships_are_distinct_objects() {
        let mut scene = Scene::default();
        let a_id = scene.add_entity(SpatialEntity::new(None)).unwrap();
        let b_id = scene.add_entity(SpatialEntity::new(None)).unwrap();
        let visual = scene
            .add_relationship(Relationship::new(a_id, b_id, RelationshipKind::Visual))
            .unwrap();
        let semantic = scene
            .add_relationship(Relationship::new(a_id, b_id, RelationshipKind::Semantic))
            .unwrap();

        assert_ne!(visual, semantic);
        assert_eq!(scene.relationships().count(), 2);
    }
}
