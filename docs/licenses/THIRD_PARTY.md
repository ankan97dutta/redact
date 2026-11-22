# Third-party licenses

Inventory stub for Redact MVP. Refresh after engine selection and dependency freezes.

| Component | Candidate | License notes |
|-----------|-----------|---------------|
| PDF redaction | `pdf_oxide` (provisional) | MIT OR Apache-2.0 — evaluate transitive deps in PoC-1 |
| PDF render (UI only) | PDF.js | Apache-2.0 — **not** used for permanent redaction |
| OCR | macOS Vision / Windows WinRT / Tesseract | Platform SDK + Apache-2.0 (Tesseract) |
| Desktop shell | Tauri 2 | MIT OR Apache-2.0 |

See `specs/001-pdf-redaction-mvp/research.md` for engine license evaluation criteria. AGPL MuPDF is **not** accepted without an explicit project license decision.
