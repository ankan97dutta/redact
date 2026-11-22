//! Canonical page rectangles in PDF user space.

use serde::{Deserialize, Serialize};

use crate::error::{AppError, ErrorCode};

/// Axis-aligned rectangle in PDF user space points.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CanonicalRect {
    pub page_index: u32,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl CanonicalRect {
    /// Create a validated rect. Requires `x1 > x0` and `y1 > y0`.
    pub fn new(page_index: u32, x0: f64, y0: f64, x1: f64, y1: f64) -> Result<Self, AppError> {
        if !(x1 > x0 && y1 > y0) {
            return Err(AppError::new(
                ErrorCode::InvalidGeometry,
                "CanonicalRect requires x1 > x0 and y1 > y0",
            ));
        }
        if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
            return Err(AppError::new(
                ErrorCode::InvalidGeometry,
                "CanonicalRect coordinates must be finite",
            ));
        }
        Ok(Self {
            page_index,
            x0,
            y0,
            x1,
            y1,
        })
    }

    pub fn width(&self) -> f64 {
        self.x1 - self.x0
    }

    pub fn height(&self) -> f64 {
        self.y1 - self.y0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_inverted() {
        assert!(CanonicalRect::new(0, 10.0, 10.0, 5.0, 20.0).is_err());
    }

    #[test]
    fn accepts_valid() {
        let r = CanonicalRect::new(0, 0.0, 0.0, 10.0, 20.0).unwrap();
        assert_eq!(r.width(), 10.0);
        assert_eq!(r.height(), 20.0);
    }
}
