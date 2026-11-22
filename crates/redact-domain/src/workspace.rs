//! Ephemeral session workspace (FR-055b) — not restored on restart.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::DocumentStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDocument {
    pub id: Uuid,
    pub source_path: String,
    pub source_hash_sha256: Option<String>,
    pub status: DocumentStatus,
    pub capability_warning: Option<String>,
}

/// In-memory session store. Dropped on process exit.
#[derive(Debug, Default, Clone)]
pub struct Workspace {
    documents: Vec<WorkspaceDocument>,
}

impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_document(&mut self, source_path: impl Into<String>) -> Uuid {
        let id = Uuid::new_v4();
        self.documents.push(WorkspaceDocument {
            id,
            source_path: source_path.into(),
            source_hash_sha256: None,
            status: DocumentStatus::Imported,
            capability_warning: None,
        });
        id
    }

    pub fn get(&self, id: Uuid) -> Option<&WorkspaceDocument> {
        self.documents.iter().find(|d| d.id == id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut WorkspaceDocument> {
        self.documents.iter_mut().find(|d| d.id == id)
    }

    pub fn documents(&self) -> &[WorkspaceDocument] {
        &self.documents
    }

    pub fn clear(&mut self) {
        self.documents.clear();
    }
}
