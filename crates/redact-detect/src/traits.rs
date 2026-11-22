//! Rule engine trait.

use redact_domain::geometry::CanonicalRect;
use redact_domain::types::Confidence;
use redact_domain::AppError;

#[derive(Debug, Clone)]
pub struct DetectionHit {
    pub rule_id: String,
    pub text: String,
    pub rect: CanonicalRect,
    pub confidence: Confidence,
    pub source: String,
}

/// Configurable sensitive-information detection.
pub trait RuleEngine: Send + Sync {
    fn detect(
        &self,
        spans: &[(String, CanonicalRect, String)],
    ) -> Result<Vec<DetectionHit>, AppError>;
}
