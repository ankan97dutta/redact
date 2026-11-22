#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod infra;

use std::sync::Mutex;

use commands::documents::AppState;
use infra::secrets::SecretHolder;

fn main() {
    if let Ok(tmp) = std::env::temp_dir().canonicalize() {
        let _ = infra::temp_files::sweep_orphans(&tmp, 24 * 60 * 60);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(SecretHolder::new()))
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::capabilities::get_capabilities,
            commands::documents::import_documents,
            commands::documents::analyze_document,
            commands::documents::get_detections,
            commands::documents::set_detection_review_state,
            commands::documents::mark_document_approved_for_export,
            commands::documents::export_documents,
            commands::documents::get_verification_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Redact");
}
