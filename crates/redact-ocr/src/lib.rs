//! Local OCR engine trait and platform adapters.

#![forbid(unsafe_code)]

pub mod adapters;
pub mod traits;

pub use traits::OcrEngine;
