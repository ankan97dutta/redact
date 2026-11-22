//! Permanent redaction via `pdf_oxide` destructive apply (fail-closed).

use pdf_oxide::editor::DocumentEditor;
use pdf_oxide::redaction::RedactionOptions;
use redact_domain::geometry::CanonicalRect;
use redact_domain::{AppError, ErrorCode};

use crate::traits::RedactionEngine;

#[derive(Debug, Default)]
pub struct PdfOxideRedact;

fn map_err(e: impl std::fmt::Display) -> AppError {
    let msg = e.to_string();
    let code = if msg.to_lowercase().contains("unsupported")
        || msg.to_lowercase().contains("type0")
        || msg.to_lowercase().contains("composite")
    {
        ErrorCode::UnsupportedConstruct
    } else {
        ErrorCode::EngineFailure
    };
    AppError::new(code, format!("pdf_oxide redact: {msg}"))
}

impl RedactionEngine for PdfOxideRedact {
    fn redact(
        &self,
        pdf_bytes: &[u8],
        regions: &[CanonicalRect],
    ) -> Result<Vec<u8>, AppError> {
        if regions.is_empty() {
            return Err(AppError::new(
                ErrorCode::EmptyApprovalSet,
                "redact requires at least one region",
            ));
        }
        let mut editor = DocumentEditor::from_bytes(pdf_bytes.to_vec()).map_err(map_err)?;
        for r in regions {
            let rect = [r.x0 as f32, r.y0 as f32, r.x1 as f32, r.y1 as f32];
            editor
                .add_redaction(r.page_index as usize, rect, Some([0.0, 0.0, 0.0]))
                .map_err(map_err)?;
        }
        editor
            .apply_redactions_destructive(RedactionOptions::default())
            .map_err(map_err)?;
        editor.save_to_bytes().map_err(map_err)
    }
}
