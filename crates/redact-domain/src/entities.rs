//! Core domain entities (US1).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::geometry::CanonicalRect;
use crate::types::{Confidence, DocumentStatus, ReviewState, VerificationOutcome};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptionState {
    None,
    Locked,
    Unlocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    NativeText,
    Ocr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub id: Uuid,
    pub document_id: Uuid,
    pub rule_id: String,
    pub category: String,
    pub review_state: ReviewState,
    pub confidence: Confidence,
    pub source: DetectionSource,
    pub rect: CanonicalRect,
    pub snippet: String,
    pub page_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualRegion {
    pub id: Uuid,
    pub document_id: Uuid,
    pub review_state: ReviewState,
    pub rect: CanonicalRect,
    pub page_index: u32,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub id: String,
    pub completed: bool,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub outcome: VerificationOutcome,
    pub checks: Vec<CheckResult>,
    pub reasons: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRecord {
    pub export_path: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub report: Option<VerificationReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub source_path: String,
    pub source_hash_sha256: Option<String>,
    pub working_copy_path: Option<String>,
    pub display_name: String,
    pub status: DocumentStatus,
    pub page_count: Option<u32>,
    pub encryption: EncryptionState,
    pub detections: Vec<Detection>,
    pub manual_regions: Vec<ManualRegion>,
    pub last_error: Option<String>,
    pub capability_warning: Option<String>,
    pub export: Option<ExportRecord>,
}

impl Document {
    pub fn new(source_path: impl Into<String>) -> Self {
        let source_path = source_path.into();
        let display_name = source_path
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or("document.pdf")
            .to_string();
        Self {
            id: Uuid::new_v4(),
            source_path,
            source_hash_sha256: None,
            working_copy_path: None,
            display_name,
            status: DocumentStatus::Imported,
            page_count: None,
            encryption: EncryptionState::None,
            detections: Vec::new(),
            manual_regions: Vec::new(),
            last_error: None,
            capability_warning: None,
            export: None,
        }
    }

    pub fn approved_detection_count(&self) -> usize {
        self.detections
            .iter()
            .filter(|d| d.review_state == ReviewState::Approved)
            .count()
    }

    pub fn approved_manual_count(&self) -> usize {
        self.manual_regions
            .iter()
            .filter(|r| r.review_state == ReviewState::Approved)
            .count()
    }
}
