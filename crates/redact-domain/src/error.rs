//! Shared application errors with stable codes.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Stable machine-readable error codes for UI and tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidGeometry,
    InvalidTransition,
    EmptyApprovalSet,
    PathConflict,
    NeedsPassword,
    UnsupportedConstruct,
    EngineFailure,
    Internal,
}

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("{code:?}: {message}")]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
