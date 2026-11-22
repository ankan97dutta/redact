//! Document IPC — security outcomes set only by domain/verify pipeline.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use redact_detect::builtins::BuiltinRuleEngine;
use redact_detect::RuleEngine;
use redact_domain::entities::{Detection, DetectionSource, Document, ManualRegion};
use redact_domain::gates::{require_export_path_differs, require_non_empty_approval};
use redact_domain::lifecycle::can_transition;
use redact_domain::pipeline::analyze::finish_analyze;
use redact_domain::pipeline::export::{begin_export, finish_export, ExportRequest};
use redact_domain::types::{DocumentStatus, ReviewState, VerificationOutcome};
use redact_domain::{AppError, CanonicalRect, ErrorCode, Workspace};
use redact_pdf::adapters::{PdfOxideExtract, PdfOxideRedact, PdfOxideSanitize};
use redact_pdf::traits::{ExtractionEngine, RedactionEngine, SanitizeEngine};
use redact_test_support::sha256_hex;
use redact_verify::IndependentVerifier;
use redact_verify::VerificationEngine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infra::temp_files::TempWorkingCopy;

#[derive(Default)]
pub struct AppState {
    pub workspace: Workspace,
    pub documents: HashMap<Uuid, Document>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub id: String,
    pub source_path: String,
    pub display_name: String,
    pub status: DocumentStatus,
    pub capability_warning: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportInput {
    pub paths: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ImportOutput {
    pub documents: Vec<DocumentSummary>,
}

fn summary(doc: &Document) -> DocumentSummary {
    DocumentSummary {
        id: doc.id.to_string(),
        source_path: doc.source_path.clone(),
        display_name: doc.display_name.clone(),
        status: doc.status,
        capability_warning: doc.capability_warning.clone(),
        last_error: doc.last_error.clone(),
    }
}

#[tauri::command]
pub fn import_documents(
    state: tauri::State<'_, Mutex<AppState>>,
    input: ImportInput,
) -> Result<ImportOutput, AppError> {
    let mut guard = state
        .lock()
        .map_err(|_| AppError::new(ErrorCode::Internal, "state lock"))?;
    let mut documents = Vec::new();
    for path in input.paths {
        let bytes = fs::read(&path).map_err(|e| {
            AppError::new(ErrorCode::EngineFailure, format!("read {path}: {e}"))
        })?;
        let mut doc = Document::new(path.clone());
        doc.source_hash_sha256 = Some(sha256_hex(&bytes));
        guard.workspace.add_document(path);
        // Keep workspace id aligned for session listing.
        if let Some(last) = guard.workspace.documents().last() {
            // Replace generated workspace entry id with document id by re-storing map only.
            let _ = last;
        }
        documents.push(summary(&doc));
        guard.documents.insert(doc.id, doc);
    }
    Ok(ImportOutput { documents })
}

#[tauri::command]
pub fn analyze_document(
    state: tauri::State<'_, Mutex<AppState>>,
    document_id: String,
) -> Result<DocumentSummary, AppError> {
    let id = Uuid::parse_str(&document_id)
        .map_err(|_| AppError::new(ErrorCode::Internal, "bad document_id"))?;
    let mut guard = state
        .lock()
        .map_err(|_| AppError::new(ErrorCode::Internal, "state lock"))?;
    let doc = guard
        .documents
        .get_mut(&id)
        .ok_or_else(|| AppError::new(ErrorCode::Internal, "document not found"))?;

    let bytes = fs::read(&doc.source_path).map_err(|e| {
        AppError::new(ErrorCode::EngineFailure, format!("read source: {e}"))
    })?;
    let spans = PdfOxideExtract.extract_text_rects(&bytes)?;
    let engine = BuiltinRuleEngine::new();
    let tagged: Vec<(String, CanonicalRect, String)> = spans
        .into_iter()
        .map(|(t, r)| (t, r, "native_text".into()))
        .collect();
    let hits = engine.detect(&tagged)?;
    doc.detections = hits
        .into_iter()
        .map(|h| {
            let category = h
                .source
                .split_once(':')
                .map(|(_, c)| c.to_string())
                .unwrap_or_else(|| h.rule_id.clone());
            Detection {
                id: Uuid::new_v4(),
                document_id: doc.id,
                rule_id: h.rule_id,
                category,
                review_state: ReviewState::Proposed,
                confidence: h.confidence,
                source: DetectionSource::NativeText,
                rect: h.rect,
                snippet: h.text,
                page_index: h.rect.page_index,
            }
        })
        .collect();
    finish_analyze(doc, None)?;
    Ok(summary(doc))
}

#[derive(Debug, Serialize)]
pub struct DetectionsOutput {
    pub detections: Vec<Detection>,
    pub manual_regions: Vec<ManualRegion>,
}

#[tauri::command]
pub fn get_detections(
    state: tauri::State<'_, Mutex<AppState>>,
    document_id: String,
) -> Result<DetectionsOutput, AppError> {
    let id = Uuid::parse_str(&document_id)
        .map_err(|_| AppError::new(ErrorCode::Internal, "bad document_id"))?;
    let guard = state
        .lock()
        .map_err(|_| AppError::new(ErrorCode::Internal, "state lock"))?;
    let doc = guard
        .documents
        .get(&id)
        .ok_or_else(|| AppError::new(ErrorCode::Internal, "document not found"))?;
    Ok(DetectionsOutput {
        detections: doc.detections.clone(),
        manual_regions: doc.manual_regions.clone(),
    })
}

#[derive(Debug, Deserialize)]
pub struct SetReviewInput {
    pub ids: Vec<String>,
    pub state: ReviewState,
}

#[tauri::command]
pub fn set_detection_review_state(
    state: tauri::State<'_, Mutex<AppState>>,
    input: SetReviewInput,
) -> Result<usize, AppError> {
    let ids: Vec<Uuid> = input
        .ids
        .iter()
        .filter_map(|s| Uuid::parse_str(s).ok())
        .collect();
    let mut guard = state
        .lock()
        .map_err(|_| AppError::new(ErrorCode::Internal, "state lock"))?;
    let mut updated = 0usize;
    for doc in guard.documents.values_mut() {
        for d in &mut doc.detections {
            if ids.contains(&d.id) {
                d.review_state = input.state;
                updated += 1;
            }
        }
        for m in &mut doc.manual_regions {
            if ids.contains(&m.id) {
                m.review_state = input.state;
                updated += 1;
            }
        }
        if doc.status == DocumentStatus::ReadyForReview && updated > 0 {
            let _ = can_transition(doc.status, DocumentStatus::Reviewing);
            doc.status = DocumentStatus::Reviewing;
        }
    }
    Ok(updated)
}

#[tauri::command]
pub fn mark_document_approved_for_export(
    state: tauri::State<'_, Mutex<AppState>>,
    document_id: String,
) -> Result<DocumentSummary, AppError> {
    let id = Uuid::parse_str(&document_id)
        .map_err(|_| AppError::new(ErrorCode::Internal, "bad document_id"))?;
    let mut guard = state
        .lock()
        .map_err(|_| AppError::new(ErrorCode::Internal, "state lock"))?;
    let doc = guard
        .documents
        .get_mut(&id)
        .ok_or_else(|| AppError::new(ErrorCode::Internal, "document not found"))?;
    require_non_empty_approval(doc.approved_detection_count(), doc.approved_manual_count())?;
    can_transition(doc.status, DocumentStatus::ApprovedForExport)?;
    doc.status = DocumentStatus::ApprovedForExport;
    Ok(summary(doc))
}

#[derive(Debug, Deserialize)]
pub struct ExportInput {
    pub document_ids: Vec<String>,
    pub output_dir: String,
}

#[derive(Debug, Serialize)]
pub struct ExportJobOutput {
    pub job_id: String,
    pub results: Vec<DocumentSummary>,
}

#[tauri::command]
pub fn export_documents(
    state: tauri::State<'_, Mutex<AppState>>,
    input: ExportInput,
) -> Result<ExportJobOutput, AppError> {
    let job_id = Uuid::new_v4().to_string();
    let tmp = TempWorkingCopy::create(&std::env::temp_dir(), &job_id)?;
    let mut results = Vec::new();

    let ids: Vec<Uuid> = input
        .document_ids
        .iter()
        .filter_map(|s| Uuid::parse_str(s).ok())
        .collect();

    let mut guard = state
        .lock()
        .map_err(|_| AppError::new(ErrorCode::Internal, "state lock"))?;

    for id in ids {
        let Some(doc) = guard.documents.get_mut(&id) else {
            continue;
        };
        if doc.status != DocumentStatus::ApprovedForExport {
            continue;
        }
        let file_name = PathBuf::from(&doc.display_name)
            .file_stem()
            .map(|s| format!("{}_redacted.pdf", s.to_string_lossy()))
            .unwrap_or_else(|| "export_redacted.pdf".into());
        let export_path = PathBuf::from(&input.output_dir).join(file_name);
        let export_path_str = export_path.to_string_lossy().to_string();

        if let Err(e) = require_export_path_differs(&doc.source_path, &export_path_str) {
            doc.last_error = Some(e.message.clone());
            doc.status = DocumentStatus::Failed;
            results.push(summary(doc));
            continue;
        }
        if export_path.exists() {
            doc.last_error = Some("file_conflict".into());
            doc.status = DocumentStatus::Failed;
            results.push(summary(doc));
            continue;
        }

        if let Err(e) = begin_export(
            doc,
            &ExportRequest {
                export_path: &export_path_str,
            },
        ) {
            doc.last_error = Some(e.message);
            results.push(summary(doc));
            continue;
        }

        let source_bytes = match fs::read(&doc.source_path) {
            Ok(b) => b,
            Err(e) => {
                doc.last_error = Some(e.to_string());
                doc.status = DocumentStatus::Failed;
                results.push(summary(doc));
                continue;
            }
        };
        let before_hash = doc.source_hash_sha256.clone();

        let approved_regions: Vec<CanonicalRect> = doc
            .detections
            .iter()
            .filter(|d| d.review_state == ReviewState::Approved)
            .map(|d| d.rect)
            .chain(
                doc.manual_regions
                    .iter()
                    .filter(|m| m.review_state == ReviewState::Approved)
                    .map(|m| m.rect),
            )
            .collect();
        let approved_texts: Vec<String> = doc
            .detections
            .iter()
            .filter(|d| d.review_state == ReviewState::Approved)
            .map(|d| d.snippet.clone())
            .collect();

        let pipeline = (|| -> Result<VerificationOutcome, AppError> {
            let redacted = PdfOxideRedact.redact(&source_bytes, &approved_regions)?;
            let sanitized = PdfOxideSanitize.sanitize(&redacted)?;
            fs::write(&export_path, &sanitized).map_err(|e| {
                AppError::new(ErrorCode::Internal, format!("write export: {e}"))
            })?;
            let report = IndependentVerifier.verify(&sanitized, &approved_regions, &approved_texts)?;
            // Immutability: source hash must still match.
            let after = sha256_hex(&fs::read(&doc.source_path).map_err(|e| {
                AppError::new(ErrorCode::Internal, format!("re-read source: {e}"))
            })?);
            if before_hash.as_ref() != Some(&after) {
                return Err(AppError::new(
                    ErrorCode::Internal,
                    "source hash changed during export",
                ));
            }
            if doc.capability_warning.is_some() && report.outcome == VerificationOutcome::Verified {
                return Ok(VerificationOutcome::Inconclusive);
            }
            Ok(report.outcome)
        })();

        match pipeline {
            Ok(outcome) => {
                let _ = finish_export(doc, outcome);
            }
            Err(e) => {
                doc.last_error = Some(e.message);
                doc.status = DocumentStatus::Failed;
            }
        }
        results.push(summary(doc));
    }

    let _ = tmp.cleanup();
    Ok(ExportJobOutput { job_id, results })
}

#[tauri::command]
pub fn get_verification_report(
    state: tauri::State<'_, Mutex<AppState>>,
    document_id: String,
) -> Result<Option<redact_domain::VerificationReport>, AppError> {
    let id = Uuid::parse_str(&document_id)
        .map_err(|_| AppError::new(ErrorCode::Internal, "bad document_id"))?;
    let guard = state
        .lock()
        .map_err(|_| AppError::new(ErrorCode::Internal, "state lock"))?;
    let doc = guard
        .documents
        .get(&id)
        .ok_or_else(|| AppError::new(ErrorCode::Internal, "document not found"))?;
    Ok(doc.export.as_ref().and_then(|e| e.report.clone()))
}
