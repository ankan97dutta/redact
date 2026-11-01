# Research: Redact Desktop PDF Redaction Secure MVP

**Date**: 2026-10-30  
**Branch**: `001-pdf-redaction-mvp`  
**Purpose**: Resolve technical unknowns before implementation; identify PoC gates.

---

## R1. Permanent PDF redaction engine

### Decision (provisional — gated by PoC-1)

**Primary candidate**: `pdf_oxide` (Rust, MIT OR Apache-2.0) behind a `RedactionEngine` trait.  
**Secondary candidate**: `pdfrs` redaction API (content-stream rewrite).  
**Not selected for redaction**: PDF.js, hand-rolled content-stream rewriter, MuPDF/PyMuPDF without AGPL or commercial license approval.

### Rationale

- Constitution requires genuine permanent removal, fail-closed behavior, and open-source-compatible licensing.
- User constraint: do not assume PDF.js provides secure redaction; do not implement content removal from scratch without justification.
- `pdf_oxide` documents ISO-aligned destructive redaction (`redaction_add` → `redaction_apply`), metadata scrubbing, hard-drop of original content objects, and fail-closed behavior on unsafe composite/Type0 fonts.
- License is MIT OR Apache-2.0 — compatible with typical permissive OSS project licenses without AGPL copyleft.
- MuPDF (and wrappers such as many “redactor” crates) is AGPL unless a commercial Artifex license is purchased — high governance risk for a distributed desktop OSS app unless the project intentionally adopts AGPL.
- `pdfrs` offers true content-stream redaction with known limitations on complex constructs; useful fallback/comparison in PoC.

### Alternatives considered

| Option | Pros | Cons |
|--------|------|------|
| MuPDF / PyMuPDF / mupdf-sys | Mature redaction API, Type3 support | AGPL or paid commercial license |
| PDFium | Strong render/extract ecosystem | Not a turnkey “true redaction” Rust API; would push toward custom rewrite |
| qpdf | Excellent structural rewrite/sanitize | Not a full redaction product API; still needs region semantics |
| Custom stream rewriter | Full control | High defect risk; violates “no from-scratch without justification” |
| Overlay-only (PDF.js annotations) | Easy UX | Explicitly forbidden as “secure redaction” |

### PoC required

**PoC-1 — Redaction engine bake-off** (blocking for Milestone M2):  
Corpus: text, CJK/composite fonts, image-under-text, form XObject, table, encrypted (unlocked copy).  
For each engine: apply known regions → extract text → assert targets gone → assert fail-closed on unsupported → license note.  
Accept primary only if fail-closed + permanence checks pass on MVP corpus subset.

---

## R2. Local OCR engine

### Decision (provisional — gated by PoC-2)

**Primary strategy**: Platform-native OCR adapters + optional Tesseract fallback.

- **macOS**: Apple Vision (on-device)
- **Windows**: Windows.Media.Ocr / WinRT OCR (on-device)
- **Fallback / CI consistency**: Tesseract (Apache-2.0) bundled or optionally linked

Expose via `OcrEngine` trait returning text + bounding boxes in a normalized page coordinate space.

### Rationale

- Spec requires local OCR, offline, no cloud.
- Native engines reduce bundle size and avoid shipping large models when OS already provides OCR.
- Tesseract is the portable OSS baseline for CI and machines where native OCR quality is insufficient; Apache-2.0 is license-friendly.
- Cloud OCR providers are out of scope for core functionality.

### Alternatives considered

| Option | Pros | Cons |
|--------|------|------|
| Tesseract-only | One codebase, Apache-2.0 | Larger ship; install/link complexity; quality variance |
| Pure-Rust OCR crates | No C++ | Immature for diverse legal/CA scans; still need PoC |
| PaddleOCR / EasyOCR | Strong accuracy | Heavy Python/native deps; packaging pain for Tauri |
| Cloud Vision APIs | Accuracy | Violates offline/privacy constitution |

### PoC required

**PoC-2 — OCR accuracy & coords** (blocking for Milestone M3):  
Scanned PAN/Aadhaar-like fixtures; measure detection recall after OCR; verify box mapping into page user space within tolerance.

---

## R3. Coordinate systems (render ↔ extract ↔ OCR ↔ redact)

### Decision

Adopt a single **canonical page space**: PDF user space for page `N`, origin bottom-left, units in PDF points (1/72"), y-up — matching PDF operators used by redaction engines.

Define explicit transforms:

1. **Render space** (PDF.js / canvas): origin top-left, y-down, CSS pixels at current zoom → convert via page viewport matrix inverse to user space.
2. **Native text quads**: from extraction engine → normalize to user space rectangles (`Rect` with `page_index`, `x0,y0,x1,y1`).
3. **OCR boxes**: usually top-left image pixels on a rasterized page → map through raster DPI and page media box to user space.
4. **Redaction input**: only canonical `Rect`s; engines never receive raw CSS/OCR pixels.

Document all transforms in `contracts/coordinates.md` and unit-test round-trips.

### Rationale

- Mismatched coordinates are a top cause of “looks redacted / still extractable” failures.
- Separating spaces prevents UI zoom from corrupting redaction geometry.

### Alternatives considered

- Use CSS pixels as canonical → couples domain to renderer; rejected.
- Use image pixel space as canonical → breaks native text PDFs; rejected.

### PoC required

**PoC-3 — Coordinate round-trip** (blocking before review UI ships):  
Known glyph positions → draw overlay in UI → redact → verify removal aligns with visual target (±2 pt tolerance configurable).

---

## R4. Detection and rule-engine abstractions

### Decision

- `RuleEngine` trait: input `TextRun[]` (normalized) → output `Detection[]` with confidence + source.
- Built-in rules as data-driven definitions (regex + optional validators for PAN/Aadhaar/GSTIN/IFSC).
- Custom rules: regex / keyword / phrase / validation hook IDs.
- Persistence: JSON rule configuration (FR-018); app settings store for enablement across restarts.
- Confidence: rules emit `high | medium | low | uncertain`; UI never auto-approves; uncertain never treated as confirmed.

### Rationale

- Matches constitution modularity and spec FR-014–FR-020.
- Keeps PII pattern logic out of React.

### Alternatives considered

- Embed rules only in TypeScript → violates UI/domain separation; rejected.
- ML NER for names/addresses in MVP → heavy, hard to verify offline; defer; use heuristic/low-confidence patterns.

---

## R5. Review and approval lifecycle

### Decision

Explicit state machine per document (see `data-model.md`):

`imported → needs_password? → analyzing → ready_for_review → reviewing → approved_for_export → exporting → verified | failed | inconclusive | unsupported`

Per detection: `proposed → approved | rejected` (manual regions enter as `proposed` then same path).

Export eligibility: domain gate requires non-empty approval set (or blocked empty export per spec assumption) and document not in terminal failure.

### Rationale

- Spec FR-028 and constitution fail-closed export labeling.

---

## R6. Batch processing and cancellation

### Decision

- `BatchJob` with per-document child tasks; shared `CancellationToken`.
- Concurrency: bounded worker pool (configurable, default low e.g. 2) to cap OCR/PDF memory.
- Cancel: cooperative between stages; partial results retained for completed docs; in-flight doc marked cancelled/failed appropriately — never verified.
- Retry: re-queue failed docs only (FR-039).

### Rationale

- Constitution XII + FR-035–041.

---

## R7. Sanitization and independent verification

### Decision

Pipeline order (non-negotiable):

1. Apply page utilities to **working copy** only  
2. Apply permanent redaction for **approved** regions  
3. **Sanitize** (metadata, JS, embedded files, hidden OC — policy toggles)  
4. Write export to new path (conflict = block)  
5. **Verify independently** by re-opening export bytes with a verifier that does not trust the redactor’s in-memory claims alone  
6. Emit `VerificationReport` with `verified | failed | inconclusive`

Verification checks (MVP documented set — not forensic-grade):

- Targeted strings/regions not present in extracted text
- Spot-check image/vector residual heuristics where engine supports
- Metadata scrub assertions when enabled
- Incomplete/errored checks ⇒ cannot be `verified`

### Rationale

- Spec FR-043–049; constitution “do not overclaim”.

### PoC required

**PoC-4 — Verify independence**: Craft export that redactor claims success but leaves extractable text; verifier must fail.

---

## R8. Secure temporary files and passwords

### Decision

- Working copies under OS temp or app cache with restrictive permissions; unique per job ID.
- Passwords held in memory (`SecretString`-style); cleared when document closed/job ends; never written to reports, logs, or exports.
- Cleanup on success, cancel, and app quit (best-effort + startup sweep of orphaned job dirs).
- Prefer decrypt-to-working-copy over keeping password for long-lived handles when engine allows.

### Rationale

- FR-054–055; constitution I/III.

---

## R9. Windows and macOS packaging

### Decision

- **Tauri 2** bundler: Windows **NSIS** (`-setup.exe`) + macOS **DMG** (and `.app`).
- Windows: prefer `webviewInstallMode: offlineInstaller` for offline-friendly installs (larger artifact).
- macOS: Developer ID signing + notarization for distribution outside App Store.
- CI: build both targets on GitHub Actions (macOS + Windows runners); store license notices for native OCR/PDF deps.
- Do not require network for app runtime after install (WebView2 must already be present or embedded).

### Rationale

- Spec platforms + constitution offline/cross-platform.

---

## R10. Rendering vs processing separation

### Decision

| Concern | Component | Must not |
|---------|-----------|----------|
| Page display / zoom / select | PDF.js (or similar) in React | Perform permanent redaction |
| Text extract | Rust `ExtractionEngine` | Depend on DOM |
| OCR | Rust `OcrEngine` | Upload images |
| Redact | Rust `RedactionEngine` | Use overlays as success |
| Sanitize | Rust `SanitizeEngine` | Skip on “verified” path |
| Verify | Rust `VerificationEngine` | Rubber-stamp redactor |

### Rationale

- User constraint and constitution II/V.

---

## License summary (engines)

| Dependency role | Candidate | License note |
|-----------------|-----------|--------------|
| Redaction | pdf_oxide | MIT OR Apache-2.0 |
| Redaction alt | pdfrs | Verify crate LICENSE in PoC |
| Redaction alt | MuPDF | AGPL or commercial — avoid unless project accepts AGPL |
| OCR | Apple Vision / WinRT | OS API; no redistributed engine license |
| OCR | Tesseract | Apache-2.0 (+ Leptonica BSD) |
| UI PDF view | PDF.js | Apache-2.0 (render only) |

Final project LICENSE (MIT/Apache/etc.) still to be chosen by maintainers; engine choices above assume a permissive app license.

---

## PoC gate summary (must pass before committing implementation)

| ID | Topic | Exit criteria |
|----|-------|---------------|
| PoC-1 | Redaction engine | Permanence + fail-closed on MVP corpus; license OK |
| PoC-2 | OCR | Offline boxes + usable recall on scan fixtures |
| PoC-3 | Coordinates | UI↔user-space↔redact round-trip within tolerance |
| PoC-4 | Verifier | Independent fail when residuals remain |
| PoC-5 | Encrypt unlock | Password PDFs unlock to working copy; wrong password clear error |
| PoC-6 | Page ops | Reorder/rotate/delete on working copy; source hash unchanged |

Unresolved research items: **none** — remaining uncertainty is intentionally captured as PoC gates, not open clarifications.
