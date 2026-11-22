//! Outcome aggregation rules (FR-044/045/046/049).

use crate::entities::CheckResult;
use crate::types::VerificationOutcome;

/// Aggregate check results into a verification outcome.
///
/// - Completed check finding failure → `Failed`
/// - Required image/vector check incomplete/skipped → `Inconclusive` (never Verified)
/// - All required checks complete and pass → `Verified`
pub fn aggregate_outcome(checks: &[CheckResult], required_ids: &[&str]) -> VerificationOutcome {
    let mut any_failed = false;
    let mut any_incomplete_required = false;

    for id in required_ids {
        let Some(check) = checks.iter().find(|c| c.id == *id) else {
            any_incomplete_required = true;
            continue;
        };
        if !check.completed {
            any_incomplete_required = true;
            continue;
        }
        if !check.passed {
            any_failed = true;
        }
    }

    // Also consider non-required completed failures (e.g. sanitize miss).
    for check in checks {
        if check.completed && !check.passed {
            any_failed = true;
        }
    }

    if any_failed {
        VerificationOutcome::Failed
    } else if any_incomplete_required {
        VerificationOutcome::Inconclusive
    } else {
        VerificationOutcome::Verified
    }
}
