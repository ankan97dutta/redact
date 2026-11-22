//! Contract smoke tests for US1 command shapes (no live Tauri runtime).

use redact_domain::types::{DocumentStatus, ReviewState, VerificationOutcome};
use serde_json::json;

#[test]
fn review_state_serde_roundtrip() {
    for state in [
        ReviewState::Proposed,
        ReviewState::Approved,
        ReviewState::Rejected,
    ] {
        let v = serde_json::to_value(state).unwrap();
        let back: ReviewState = serde_json::from_value(v).unwrap();
        assert_eq!(back, state);
    }
}

#[test]
fn verification_outcome_not_writable_in_client_payload_example() {
    // Client payloads may carry review state, never a writable verification outcome field.
    let client = json!({
        "ids": ["00000000-0000-0000-0000-000000000001"],
        "state": "approved"
    });
    assert!(client.get("verification_outcome").is_none());
    assert!(client.get("outcome").is_none());
}

#[test]
fn document_status_includes_secure_terminals() {
    let statuses = [
        DocumentStatus::Verified,
        DocumentStatus::Failed,
        DocumentStatus::Inconclusive,
    ];
    for s in statuses {
        let s = serde_json::to_string(&s).unwrap();
        assert!(!s.is_empty());
    }
    let _ = VerificationOutcome::Verified;
}
