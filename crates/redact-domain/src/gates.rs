//! Export eligibility gates (FR-028a and path rules).

use crate::error::{AppError, ErrorCode};

/// Block export when the approval set is empty (FR-028a).
pub fn require_non_empty_approval(
    approved_detections: usize,
    approved_manual_regions: usize,
) -> Result<(), AppError> {
    if approved_detections + approved_manual_regions == 0 {
        return Err(AppError::new(
            ErrorCode::EmptyApprovalSet,
            "Export blocked: at least one approved detection or manual region is required",
        ));
    }
    Ok(())
}

/// Refuse export targets that would replace the source path.
pub fn require_export_path_differs(source: &str, export: &str) -> Result<(), AppError> {
    let s = normalize_path(source);
    let e = normalize_path(export);
    if s == e {
        return Err(AppError::new(
            ErrorCode::PathConflict,
            "Export path must not overwrite the original source document",
        ));
    }
    Ok(())
}

fn normalize_path(p: &str) -> String {
    p.replace('\\', "/").trim_end_matches('/').to_lowercase()
}
