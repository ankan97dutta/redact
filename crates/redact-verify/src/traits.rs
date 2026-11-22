//! Verification engine — sole producer of VerificationOutcome.

use redact_domain::entities::VerificationReport;
use redact_domain::geometry::CanonicalRect;
use redact_domain::AppError;

/// Independent verification after sanitize.
pub trait VerificationEngine: Send + Sync {
    fn verify(
        &self,
        export_bytes: &[u8],
        approved_regions: &[CanonicalRect],
        approved_texts: &[String],
    ) -> Result<VerificationReport, AppError>;
}
