use redact_domain::entities::Document;
use redact_domain::pipeline::export::{begin_export, finish_export, ExportRequest};
use redact_domain::types::{DocumentStatus, ReviewState, VerificationOutcome};
use redact_domain::ErrorCode;

#[test]
fn unsupported_construct_never_verified() {
    let mut doc = Document::new("/tmp/source.pdf");
    doc.status = DocumentStatus::ApprovedForExport;
    doc.capability_warning = Some("Type3 font composite unsupported".into());
    doc.detections.push(redact_domain::Detection {
        id: uuid::Uuid::new_v4(),
        document_id: doc.id,
        rule_id: "pan".into(),
        category: "pan".into(),
        review_state: ReviewState::Approved,
        confidence: redact_domain::Confidence::High,
        source: redact_domain::DetectionSource::NativeText,
        rect: redact_domain::CanonicalRect::new(0, 0.0, 0.0, 10.0, 10.0).unwrap(),
        snippet: "AAAPA1234A".into(),
        page_index: 0,
    });

    begin_export(
        &mut doc,
        &ExportRequest {
            export_path: "/tmp/out.pdf",
        },
    )
    .unwrap();

    let err = finish_export(&mut doc, VerificationOutcome::Verified).unwrap_err();
    assert_eq!(err.code, ErrorCode::UnsupportedConstruct);
    assert_ne!(doc.status, DocumentStatus::Verified);
}
