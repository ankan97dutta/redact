//! Document status transition guards.

use crate::error::{AppError, ErrorCode};
use crate::types::DocumentStatus;

/// Returns `Ok(())` if `from → to` is allowed.
///
/// `verified` is reachable **only** from `exporting` after successful verification.
pub fn can_transition(from: DocumentStatus, to: DocumentStatus) -> Result<(), AppError> {
    use DocumentStatus::*;

    if from == to {
        return Ok(());
    }

    let allowed = matches!(
        (from, to),
        (Imported, NeedsPassword)
            | (Imported, Analyzing)
            | (Imported, Unsupported)
            | (Imported, Failed)
            | (NeedsPassword, Analyzing)
            | (NeedsPassword, Failed)
            | (NeedsPassword, Unsupported)
            | (NeedsPassword, Cancelled)
            | (Analyzing, ReadyForReview)
            | (Analyzing, Failed)
            | (Analyzing, Unsupported)
            | (Analyzing, Cancelled)
            | (ReadyForReview, Reviewing)
            | (ReadyForReview, ApprovedForExport)
            | (ReadyForReview, Cancelled)
            | (Reviewing, ApprovedForExport)
            | (Reviewing, ReadyForReview)
            | (Reviewing, Cancelled)
            | (ApprovedForExport, Exporting)
            | (ApprovedForExport, Reviewing)
            | (ApprovedForExport, Cancelled)
            | (Exporting, Verified)
            | (Exporting, Failed)
            | (Exporting, Inconclusive)
            | (Exporting, Cancelled)
            | (Failed, Analyzing)
            | (Failed, ApprovedForExport)
            | (Inconclusive, Analyzing)
            | (Inconclusive, ApprovedForExport)
    );

    if allowed {
        Ok(())
    } else {
        Err(AppError::new(
            ErrorCode::InvalidTransition,
            format!("forbidden transition {from:?} → {to:?}"),
        ))
    }
}

/// Client / UI must never set `verified` directly.
pub fn assert_client_cannot_set_verified(to: DocumentStatus) -> Result<(), AppError> {
    if to == DocumentStatus::Verified {
        Err(AppError::new(
            ErrorCode::InvalidTransition,
            "UI cannot set verification outcome to verified",
        ))
    } else {
        Ok(())
    }
}
