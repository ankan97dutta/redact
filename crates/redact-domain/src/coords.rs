//! Coordinate transforms between render CSS, PDF user space, and raster pixels.

use crate::geometry::CanonicalRect;

/// Page size in PDF user-space points.
#[derive(Debug, Clone, Copy)]
pub struct PageSizePts {
    pub width: f64,
    pub height: f64,
}

/// CSS render viewport size in CSS pixels (PDF.js style).
#[derive(Debug, Clone, Copy)]
pub struct RenderSizeCss {
    pub width: f64,
    pub height: f64,
}

/// Convert a CSS-space rect (origin top-left) to PDF user space (origin bottom-left).
pub fn render_css_to_user(
    page_index: u32,
    css_x0: f64,
    css_y0: f64,
    css_x1: f64,
    css_y1: f64,
    page: PageSizePts,
    render: RenderSizeCss,
) -> Result<CanonicalRect, crate::error::AppError> {
    let sx = page.width / render.width;
    let sy = page.height / render.height;
    let x0 = css_x0 * sx;
    let x1 = css_x1 * sx;
    // CSS y grows downward; PDF user y grows upward.
    let y1 = page.height - css_y0 * sy;
    let y0 = page.height - css_y1 * sy;
    CanonicalRect::new(page_index, x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1))
}

/// Convert PDF user-space rect to CSS render coordinates (origin top-left).
pub fn user_to_render_css(
    rect: &CanonicalRect,
    page: PageSizePts,
    render: RenderSizeCss,
) -> (f64, f64, f64, f64) {
    let sx = render.width / page.width;
    let sy = render.height / page.height;
    let x0 = rect.x0 * sx;
    let x1 = rect.x1 * sx;
    let y0 = (page.height - rect.y1) * sy;
    let y1 = (page.height - rect.y0) * sy;
    (x0, y0, x1, y1)
}

/// Map raster pixel box (origin top-left) into PDF user space.
pub fn raster_px_to_user(
    page_index: u32,
    px_x0: f64,
    px_y0: f64,
    px_x1: f64,
    px_y1: f64,
    page: PageSizePts,
    raster_w: f64,
    raster_h: f64,
) -> Result<CanonicalRect, crate::error::AppError> {
    let sx = page.width / raster_w;
    let sy = page.height / raster_h;
    let x0 = px_x0 * sx;
    let x1 = px_x1 * sx;
    let y1 = page.height - px_y0 * sy;
    let y0 = page.height - px_y1 * sy;
    CanonicalRect::new(page_index, x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1))
}
