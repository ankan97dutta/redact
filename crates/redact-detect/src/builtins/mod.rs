//! Built-in India-centric and common professional detectors (MVP subset).

mod patterns;

use redact_domain::geometry::CanonicalRect;
use redact_domain::types::Confidence;
use redact_domain::AppError;
use regex::Regex;

use crate::traits::{DetectionHit, RuleEngine};

pub struct BuiltinRuleEngine {
    rules: Vec<BuiltinRule>,
}

struct BuiltinRule {
    id: &'static str,
    category: &'static str,
    re: Regex,
    confidence: Confidence,
}

impl Default for BuiltinRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltinRuleEngine {
    pub fn new() -> Self {
        let rules = patterns::RULE_SPECS
            .iter()
            .filter_map(|spec| {
                Regex::new(spec.pattern).ok().map(|re| BuiltinRule {
                    id: spec.id,
                    category: spec.category,
                    re,
                    confidence: spec.confidence,
                })
            })
            .collect();
        Self { rules }
    }
}

impl RuleEngine for BuiltinRuleEngine {
    fn detect(
        &self,
        spans: &[(String, CanonicalRect, String)],
    ) -> Result<Vec<DetectionHit>, AppError> {
        let mut hits = Vec::new();
        for (text, rect, source) in spans {
            for rule in &self.rules {
                for m in rule.re.find_iter(text) {
                    hits.push(DetectionHit {
                        rule_id: rule.id.into(),
                        text: m.as_str().to_string(),
                        rect: *rect,
                        confidence: rule.confidence,
                        source: format!("{}:{}", source, rule.category),
                    });
                }
            }
        }
        Ok(hits)
    }
}
