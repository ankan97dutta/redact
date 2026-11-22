//! PDF extract, redact, and sanitize traits and adapters.

#![forbid(unsafe_code)]

pub mod adapters;
pub mod traits;

pub use traits::{ExtractionEngine, RedactionEngine, SanitizeEngine};
