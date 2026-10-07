#![forbid(unsafe_code)]

use neurite_core::EntityId;
use neurite_scene::Scene;
use std::collections::BTreeMap;
use std::path::Path;

const MAX_IMPORTED_TEXT_BYTES: u64 = 1_048_576;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceFile {
    pub name: String,
    /// Immutable copy of imported text, independent of the source path.
    pub content: String,
}

#[derive(Debug)]
pub enum WorkspaceFileError {
    InvalidName,
    TooLarge,
    Io(std::io::Error),
}

#[derive(Debug)]
pub struct Workspace {
    pub id: String,
    pub schema_version: u32,
    pub scene: Scene,
    pub notes: BTreeMap<EntityId, String>,
    pub files: BTreeMap<EntityId, WorkspaceFile>,
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
            files: BTreeMap::new(),
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

    pub fn set_file(
        &mut self,
        entity: EntityId,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<(), WorkspaceFileError> {
        let name = name.into();
        let content = content.into();
        if name.is_empty()
            || name.len() > 255
            || name == "."
            || name == ".."
            || name.chars().any(char::is_control)
            || Path::new(&name).file_name().and_then(|v| v.to_str()) != Some(name.as_str())
            || name.contains('/')
            || name.contains('\\')
        {
            return Err(WorkspaceFileError::InvalidName);
        }
        if content.len() as u64 > MAX_IMPORTED_TEXT_BYTES {
            return Err(WorkspaceFileError::TooLarge);
        }
        self.files.insert(entity, WorkspaceFile { name, content });
        self.mark_dirty();
        Ok(())
    }

    pub fn import_text_file(
        &mut self,
        entity: EntityId,
        path: &Path,
    ) -> Result<(), WorkspaceFileError> {
        let metadata = std::fs::metadata(path).map_err(WorkspaceFileError::Io)?;
        if !metadata.is_file() {
            return Err(WorkspaceFileError::InvalidName);
        }
        if metadata.len() > MAX_IMPORTED_TEXT_BYTES {
            return Err(WorkspaceFileError::TooLarge);
        }
        let name = path
            .file_name()
            .and_then(|v| v.to_str())
            .ok_or(WorkspaceFileError::InvalidName)?
            .to_owned();
        let content = std::fs::read_to_string(path).map_err(WorkspaceFileError::Io)?;
        self.set_file(entity, name, content)
    }

    pub fn delete_file(&mut self, entity: EntityId) -> Option<WorkspaceFile> {
        let removed = self.files.remove(&entity);
        if removed.is_some() {
            self.mark_dirty();
        }
        removed
    }
}
