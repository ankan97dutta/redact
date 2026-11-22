//! Independent verification — sole producer of verification outcomes.

#![forbid(unsafe_code)]

pub mod engine;
pub mod traits;

pub use engine::{outcome_from_checks, IndependentVerifier};
pub use traits::VerificationEngine;
