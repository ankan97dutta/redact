//! Build-scoped Capabilities summary (FR-004a).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Supported,
    Limited,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineInfo {
    pub role: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityEntry {
    pub id: String,
    pub status: CapabilityStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitiesSummary {
    pub schema_version: String,
    pub app_version: String,
    pub engines: Vec<EngineInfo>,
    pub document_classes: Vec<CapabilityEntry>,
    pub verification_checks: Vec<CapabilityEntry>,
    pub limitations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

impl CapabilitiesSummary {
    /// Minimal honest summary for builds before engines are registered.
    pub fn placeholder(app_version: &str) -> Self {
        Self {
            schema_version: "0.1.0".into(),
            app_version: app_version.into(),
            engines: vec![],
            document_classes: vec![CapabilityEntry {
                id: "text_pdf".into(),
                status: CapabilityStatus::Limited,
                detail: Some("Engine adapters pending PoC selection".into()),
            }],
            verification_checks: vec![CapabilityEntry {
                id: "text_residue".into(),
                status: CapabilityStatus::Limited,
                detail: Some("Verifier pending PoC-4".into()),
            }],
            limitations: vec![
                "Capabilities are provisional until PoC-1/PoC-4 engine selection is complete."
                    .into(),
            ],
            generated_at: Utc::now(),
        }
    }
}
