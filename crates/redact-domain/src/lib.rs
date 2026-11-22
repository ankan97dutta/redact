//! Pure domain types, gates, and orchestration for Redact.
//! No Tauri or PDF engine dependencies.

#![forbid(unsafe_code)]

pub mod capabilities;
pub mod coords;
pub mod entities;
pub mod error;
pub mod gates;
pub mod geometry;
pub mod lifecycle;
pub mod pipeline;
pub mod types;
pub mod workspace;

pub use capabilities::{CapabilitiesSummary, CapabilityEntry, CapabilityStatus, EngineInfo};
pub use entities::{
    CheckResult, Detection, DetectionSource, Document, EncryptionState, ExportRecord, ManualRegion,
    VerificationReport,
};
pub use error::{AppError, ErrorCode};
pub use geometry::CanonicalRect;
pub use types::{Confidence, DocumentStatus, ReviewState, VerificationOutcome};
pub use workspace::Workspace;
