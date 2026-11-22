//! OCR engine trait.

use redact_domain::geometry::CanonicalRect;
use redact_domain::AppError;

#[derive(Debug, Clone)]
pub struct OcrSpan {
    pub text: String,
    pub rect: CanonicalRect,
}

/// Local OCR — MUST NOT require network.
pub trait OcrEngine: Send + Sync {
    fn recognize_page(
        &self,
        raster_png: &[u8],
        page_index: u32,
        page_width_pts: f64,
        page_height_pts: f64,
    ) -> Result<Vec<OcrSpan>, AppError>;
}
