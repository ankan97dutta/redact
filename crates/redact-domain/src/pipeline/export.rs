//! Export pipeline order: redact → sanitize → write → verify (orchestration stub).

use crate::entities::Document;
use crate::error::{AppError, ErrorCode};
use crate::gates::{require_export_path_differs, require_non_empty_approval};
use crate::lifecycle::can_transition;
use crate::types::{DocumentStatus, VerificationOutcome};

pub struct ExportRequest<'a> {
    pub export_path: &'a str,
}

/// Validate eligibility and transition into `exporting`.
pub fn begin_export(doc: &mut Document, req: &ExportRequest<'_>) -> Result<(), AppError> {
    require_non_empty_approval(doc.approved_detection_count(), doc.approved_manual_count())?;
    require_export_path_differs(&doc.source_path, req.export_path)?;

    if doc.status != DocumentStatus::ApprovedForExport {
        can_transition(doc.status, DocumentStatus::ApprovedForExport)?;
        doc.status = DocumentStatus::ApprovedForExport;
    }
    can_transition(doc.status, DocumentStatus::Exporting)?;
    doc.status = DocumentStatus::Exporting;
    Ok(())
}

/// Apply verification outcome to document status (never verified on unsupported warning path).
pub fn finish_export(
    doc: &mut Document,
    outcome: VerificationOutcome,
) -> Result<(), AppError> {
    if doc.capability_warning.is_some() && outcome == VerificationOutcome::Verified {
        return Err(AppError::new(
            ErrorCode::UnsupportedConstruct,
            "export with unsupported constructs cannot be labeled verified",
        ));
    }
    let to = match outcome {
        VerificationOutcome::Verified => DocumentStatus::Verified,
        VerificationOutcome::Failed => DocumentStatus::Failed,
        VerificationOutcome::Inconclusive => DocumentStatus::Inconclusive,
    };
    can_transition(doc.status, to)?;
    doc.status = to;
    Ok(())
}
