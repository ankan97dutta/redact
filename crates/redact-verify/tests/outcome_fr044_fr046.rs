use redact_domain::entities::CheckResult;
use redact_domain::types::VerificationOutcome;
use redact_verify::outcome_from_checks;

fn check(id: &str, completed: bool, passed: bool) -> CheckResult {
    CheckResult {
        id: id.into(),
        completed,
        passed,
        detail: String::new(),
    }
}

#[test]
fn image_incomplete_inconclusive() {
    let checks = vec![
        check("text_residue", true, true),
        check("sanitize_targets", true, true),
        check("image_residual", false, false),
        check("vector_residual", true, true),
    ];
    assert_eq!(
        outcome_from_checks(&checks),
        VerificationOutcome::Inconclusive
    );
}

#[test]
fn residue_failed() {
    let checks = vec![
        check("text_residue", true, false),
        check("sanitize_targets", true, true),
        check("image_residual", true, true),
        check("vector_residual", true, true),
    ];
    assert_eq!(outcome_from_checks(&checks), VerificationOutcome::Failed);
}
