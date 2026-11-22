//! Placeholder adapter until the real `pdf_oxide` dependency is wired in US1 (T033).
//!
//! Selected by PoC-1 as primary redaction engine candidate (MIT OR Apache-2.0).

use redact_domain::geometry::CanonicalRect;
use redact_domain::{AppError, ErrorCode};

use crate::traits::{ExtractionEngine, RedactionEngine, SanitizeEngine};

#[derive(Debug, Default)]
pub struct PdfOxideStub;

impl ExtractionEngine for PdfOxideStub {
    fn extract_text_rects(
        &self,
        _pdf_bytes: &[u8],
    ) -> Result<Vec<(String, CanonicalRect)>, AppError> {
        Err(AppError::new(
            ErrorCode::EngineFailure,
            "pdf_oxide extract adapter not yet implemented (PoC-1 selected; US1 T032)",
        ))
    }
}

impl RedactionEngine for PdfOxideStub {
    fn redact(
        &self,
        _pdf_bytes: &[u8],
        _regions: &[CanonicalRect],
    ) -> Result<Vec<u8>, AppError> {
        Err(AppError::new(
            ErrorCode::EngineFailure,
            "pdf_oxide redact adapter not yet implemented (PoC-1 selected; US1 T033)",
        ))
    }
}

impl SanitizeEngine for PdfOxideStub {
    fn sanitize(&self, _pdf_bytes: &[u8]) -> Result<Vec<u8>, AppError> {
        Err(AppError::new(
            ErrorCode::EngineFailure,
            "pdf_oxide sanitize adapter not yet implemented (PoC-1 selected; US1 T034)",
        ))
    }
}
