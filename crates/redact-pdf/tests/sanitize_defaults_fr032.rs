use redact_pdf::adapters::PdfOxideSanitize;
use redact_pdf::traits::SanitizeEngine;
use redact_test_support::minimal_text_pdf;

#[test]
fn sanitize_returns_pdf_bytes() {
    let input = minimal_text_pdf("sanitize me AAAPA1234A");
    let engine = PdfOxideSanitize;
    let out = engine.sanitize(&input).expect("sanitize");
    assert!(out.starts_with(b"%PDF"));
    // Sanitization must not fail-open to empty.
    assert!(out.len() > 64);
}
