use redact_domain::CapabilitiesSummary;

/// FR-004a: build-scoped capabilities summary.
#[tauri::command]
pub fn get_capabilities() -> CapabilitiesSummary {
    CapabilitiesSummary::placeholder(env!("CARGO_PKG_VERSION"))
}
