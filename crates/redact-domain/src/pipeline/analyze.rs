//! Analyze → detect orchestration (stub for US1 wiring).

use crate::entities::Document;
use crate::error::{AppError, ErrorCode};
use crate::lifecycle::can_transition;
use crate::types::DocumentStatus;

/// Mark document analyzing → ready_for_review, attaching capability warnings when provided.
pub fn finish_analyze(
    doc: &mut Document,
    capability_warning: Option<String>,
) -> Result<(), AppError> {
    if doc.status == DocumentStatus::Imported || doc.status == DocumentStatus::NeedsPassword {
        can_transition(doc.status, DocumentStatus::Analyzing)?;
        doc.status = DocumentStatus::Analyzing;
    }
    can_transition(doc.status, DocumentStatus::ReadyForReview)?;
    doc.status = DocumentStatus::ReadyForReview;
    doc.capability_warning = capability_warning;
    Ok(())
}

/// Documents with unsupported secure-redaction constructs must never become verified later.
pub fn note_unsupported_construct(doc: &mut Document, reason: impl Into<String>) {
    doc.capability_warning = Some(reason.into());
}

pub fn ensure_not_claiming_verified_with_warning(doc: &Document) -> Result<(), AppError> {
    if doc.capability_warning.is_some() && doc.status == DocumentStatus::Verified {
        return Err(AppError::new(
            ErrorCode::UnsupportedConstruct,
            "unsupported-construct documents cannot be verified",
        ));
    }
    Ok(())
}
