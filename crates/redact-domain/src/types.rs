//! Canonical status and review enums (FR-003).

use serde::{Deserialize, Serialize};

/// Per-document processing status. Codes are stable for automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    Imported,
    NeedsPassword,
    Analyzing,
    ReadyForReview,
    Reviewing,
    ApprovedForExport,
    Exporting,
    Verified,
    Failed,
    Inconclusive,
    Unsupported,
    Cancelled,
}

/// Detection confidence band.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    High,
    Medium,
    Low,
    Uncertain,
}

/// User review state for detections / manual regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    Proposed,
    Approved,
    Rejected,
}

/// Verification outcome — set only by the verification engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationOutcome {
    Verified,
    Failed,
    Inconclusive,
}
