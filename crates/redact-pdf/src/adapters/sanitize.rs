//! MVP default sanitization (FR-032) via `pdf_oxide`.

use pdf_oxide::editor::DocumentEditor;
use pdf_oxide::redaction::RedactionOptions;
use redact_domain::{AppError, ErrorCode};

use crate::traits::SanitizeEngine;

#[derive(Debug, Default)]
pub struct PdfOxideSanitize;

fn map_err(e: impl std::fmt::Display) -> AppError {
    AppError::new(ErrorCode::EngineFailure, format!("pdf_oxide sanitize: {e}"))
}

impl SanitizeEngine for PdfOxideSanitize {
    fn sanitize(&self, pdf_bytes: &[u8]) -> Result<Vec<u8>, AppError> {
        let mut editor = DocumentEditor::from_bytes(pdf_bytes.to_vec()).map_err(map_err)?;
        // Defaults scrub metadata, JS, embedded files (FR-032).
        editor
            .sanitize_document(RedactionOptions::default())
            .map_err(map_err)?;
        editor.save_to_bytes().map_err(map_err)
    }
}
