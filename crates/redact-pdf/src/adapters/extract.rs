//! Native text extraction via `pdf_oxide`.

use pdf_oxide::editor::DocumentEditor;
use redact_domain::geometry::CanonicalRect;
use redact_domain::{AppError, ErrorCode};

use crate::traits::ExtractionEngine;

#[derive(Debug, Default)]
pub struct PdfOxideExtract;

fn map_err(e: impl std::fmt::Display) -> AppError {
    AppError::new(ErrorCode::EngineFailure, format!("pdf_oxide extract: {e}"))
}

impl ExtractionEngine for PdfOxideExtract {
    fn extract_text_rects(
        &self,
        pdf_bytes: &[u8],
    ) -> Result<Vec<(String, CanonicalRect)>, AppError> {
        let editor = DocumentEditor::from_bytes(pdf_bytes.to_vec()).map_err(map_err)?;
        let page_count = editor.current_page_count();
        let mut out = Vec::new();

        for page_index in 0..page_count {
            let spans = editor
                .source()
                .extract_spans(page_index)
                .map_err(map_err)?;
            for span in spans {
                let text = span.text.trim();
                if text.is_empty() {
                    continue;
                }
                // TextSpan bbox: top-left origin in PDF user-space points.
                let x0 = f64::from(span.bbox.x);
                let top = f64::from(span.bbox.y);
                let w = f64::from(span.bbox.width).abs();
                let h = f64::from(span.bbox.height).abs();
                let y1 = top;
                let y0 = top - h;
                match CanonicalRect::new(page_index as u32, x0, y0.min(y1), x0 + w, y0.max(y1)) {
                    Ok(rect) => out.push((text.to_string(), rect)),
                    Err(_) => continue,
                }
            }
        }
        Ok(out)
    }
}
