# Contract: Coordinate Spaces

## Canonical space

**Name**: `PdfUserSpaceRect`  
**Origin**: bottom-left of the page’s crop/media box as used by the redaction engine  
**Axes**: x right, y up  
**Units**: PDF points (1/72 inch)  
**Page index**: 0-based

All detections, manual regions, and redaction API inputs MUST use this space.

## Other spaces

| Space | Origin | Y | Used by |
|-------|--------|---|---------|
| RenderCss | top-left of canvas | down | PDF.js viewer overlays |
| RasterPx | top-left of rendered page image | down | OCR input |
| EngineNative | engine-specific | varies | Immediately converted at adapter boundary |

## Required transforms (adapters)

1. `render_css_to_user(page, css_rect, viewport) -> PdfUserSpaceRect`
2. `user_to_render_css(page, user_rect, viewport) -> CssRect`
3. `raster_px_to_user(page, px_rect, dpi, media_box) -> PdfUserSpaceRect`

Round-trip tests MUST keep error ≤ configured tolerance (default **2 pt**) on PoC fixtures.

## Invariants

- UI zoom/pan MUST NOT change stored canonical rects; only viewport mapping changes.
- OCR and native text detections share the same canonical type.
- Redaction engines receive only canonical rects.