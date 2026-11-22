//! Detection rule engine and built-in patterns.

#![forbid(unsafe_code)]

pub mod builtins;
pub mod traits;

pub use builtins::BuiltinRuleEngine;
pub use traits::{DetectionHit, RuleEngine};
