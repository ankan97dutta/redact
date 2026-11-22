use redact_domain::entities::CheckResult;
use redact_domain::pipeline::aggregate_outcome;
use redact_domain::types::VerificationOutcome;

fn check(id: &str, completed: bool, passed: bool) -> CheckResult {
    CheckResult {
        id: id.into(),
        completed,
        passed,
        detail: String::new(),
    }
}

#[test]
fn incomplete_image_vector_yields_inconclusive() {
    let checks = vec![
        check("text_residue", true, true),
        check("sanitize_targets", true, true),
        check("image_residual", false, false),
        check("vector_residual", true, true),
    ];
    let outcome = aggregate_outcome(
        &checks,
        &["text_residue", "sanitize_targets", "image_residual", "vector_residual"],
    );
    assert_eq!(outcome, VerificationOutcome::Inconclusive);
}

#[test]
fn residue_hit_yields_failed() {
    let checks = vec![
        check("text_residue", true, false),
        check("sanitize_targets", true, true),
        check("image_residual", true, true),
        check("vector_residual", true, true),
    ];
    let outcome = aggregate_outcome(
        &checks,
        &["text_residue", "sanitize_targets", "image_residual", "vector_residual"],
    );
    assert_eq!(outcome, VerificationOutcome::Failed);
}

#[test]
fn all_pass_verified() {
    let checks = vec![
        check("text_residue", true, true),
        check("sanitize_targets", true, true),
        check("image_residual", true, true),
        check("vector_residual", true, true),
    ];
    let outcome = aggregate_outcome(
        &checks,
        &["text_residue", "sanitize_targets", "image_residual", "vector_residual"],
    );
    assert_eq!(outcome, VerificationOutcome::Verified);
}

#[test]
fn sanitize_miss_not_verified() {
    let checks = vec![
        check("text_residue", true, true),
        check("sanitize_targets", true, false),
        check("image_residual", true, true),
        check("vector_residual", true, true),
    ];
    let outcome = aggregate_outcome(
        &checks,
        &["text_residue", "sanitize_targets", "image_residual", "vector_residual"],
    );
    assert_eq!(outcome, VerificationOutcome::Failed);
}
