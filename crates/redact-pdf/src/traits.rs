//! PDF engine traits.

use redact_domain::geometry::CanonicalRect;
use redact_domain::AppError;

/// Native text / structure extraction.
pub trait ExtractionEngine: Send + Sync {
    fn extract_text_rects(
        &self,
        pdf_bytes: &[u8],
    ) -> Result<Vec<(String, CanonicalRect)>, AppError>;
}

/// Permanent content removal for approved regions.
pub trait RedactionEngine: Send + Sync {
    fn redact(
        &self,
        pdf_bytes: &[u8],
        regions: &[CanonicalRect],
    ) -> Result<Vec<u8>, AppError>;
}

/// MVP default sanitization (FR-032).
pub trait SanitizeEngine: Send + Sync {
    fn sanitize(&self, pdf_bytes: &[u8]) -> Result<Vec<u8>, AppError>;
}
