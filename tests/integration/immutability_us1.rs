//! Source bytes must remain unchanged after a redact→sanitize round-trip on a copy.

use redact_pdf::adapters::{PdfOxideRedact, PdfOxideSanitize};
use redact_pdf::traits::{RedactionEngine, SanitizeEngine};
use redact_domain::CanonicalRect;
use redact_test_support::{sha256_hex, text_pan_sample};

#[test]
fn source_hash_unchanged_after_working_copy_pipeline() {
    let (source, _) = text_pan_sample();
    let before = sha256_hex(&source);

    // Work on a clone — never mutate `source`.
    let working = source.clone();
    let region = CanonicalRect::new(0, 70.0, 700.0, 400.0, 750.0).unwrap();
    let redacted = PdfOxideRedact
        .redact(&working, &[region])
        .unwrap_or_else(|_| working.clone());
    let _ = PdfOxideSanitize.sanitize(&redacted);

    assert_eq!(sha256_hex(&source), before);
    assert_eq!(source, working);
}
