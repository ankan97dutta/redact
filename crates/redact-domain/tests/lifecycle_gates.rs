use redact_domain::gates::{require_export_path_differs, require_non_empty_approval};
use redact_domain::lifecycle::{assert_client_cannot_set_verified, can_transition};
use redact_domain::types::DocumentStatus;
use redact_domain::ErrorCode;

#[test]
fn exporting_to_verified_allowed() {
    assert!(can_transition(DocumentStatus::Exporting, DocumentStatus::Verified).is_ok());
}

#[test]
fn reviewing_to_verified_forbidden() {
    let err = can_transition(DocumentStatus::Reviewing, DocumentStatus::Verified).unwrap_err();
    assert_eq!(err.code, ErrorCode::InvalidTransition);
}

#[test]
fn client_cannot_set_verified() {
    let err = assert_client_cannot_set_verified(DocumentStatus::Verified).unwrap_err();
    assert_eq!(err.code, ErrorCode::InvalidTransition);
}

#[test]
fn empty_approval_blocked() {
    let err = require_non_empty_approval(0, 0).unwrap_err();
    assert_eq!(err.code, ErrorCode::EmptyApprovalSet);
    assert!(err.message.contains("at least one approved"));
}

#[test]
fn non_empty_approval_ok() {
    assert!(require_non_empty_approval(1, 0).is_ok());
    assert!(require_non_empty_approval(0, 1).is_ok());
}

#[test]
fn source_export_same_path_blocked() {
    let err = require_export_path_differs("/tmp/a.pdf", "/tmp/a.pdf").unwrap_err();
    assert_eq!(err.code, ErrorCode::PathConflict);
}
