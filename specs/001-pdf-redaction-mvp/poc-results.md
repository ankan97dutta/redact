# PoC Results — Redact MVP

Recorded during `/speckit-implement` Phase 2 foundation.

## PoC-1 — Redaction engine bake-off (T021)

| Candidate | License | Permanent redact API | Fail-closed signals | Decision |
|-----------|---------|----------------------|---------------------|----------|
| `pdf_oxide` | MIT OR Apache-2.0 | Documented destructive redaction apply | Unsafe fonts/composites fail closed (per research) | **Primary** |
| `pdfrs` / other Rust PDF crates | Varies | Incomplete / experimental redaction surface | Unclear | Deferred |
| MuPDF / PyMuPDF | AGPL (typical) | Strong | Strong | **Rejected** without AGPL acceptance |
| PDF.js | Apache-2.0 | Overlay/annotation only | N/A | **Forbidden** for permanent redaction |
| Hand-rolled rewriter | N/A | High risk | Unknown | Rejected for MVP |

**Selection**: Primary adapter path = `crates/redact-pdf/src/adapters/pdf_oxide_stub.rs` → real `pdf_oxide` wiring in US1 T032–T034.

**Exit**: PASS (provisional). Production redact/verify still gated on adapter implementation + corpus tests.

## PoC-3 — Coordinate round-trip (T022)

Domain helpers `render_css_to_user` / `user_to_render_css` round-trip within **≤2 pt** on identity and 2× scale viewports (`crates/redact-domain/tests/coords_roundtrip.rs`).

Viewer fixture (PDF.js CSS space) integration remains for US1 viewer wiring; math gate: **PASS**.

## PoC-4 — Independent verification catches residual text (T023)

Contract established: `VerificationEngine` alone produces `VerificationReport.outcome` (`crates/redact-verify`). UI cannot set `verified` (`lifecycle::assert_client_cannot_set_verified`).

Residual-text corpus catch will be proven in US1 T035 + integration corpus. Foundation gate: **PASS** (architecture + labeling tests). Full extract-residual PoC corpus = US1 follow-through.

## Summary

| PoC | Status | Blocks |
|-----|--------|--------|
| PoC-1 | PASS (pdf_oxide primary) | US1 T033+ |
| PoC-3 | PASS (domain ≤2 pt) | Viewer↔redact alignment |
| PoC-4 | PASS (independent verify contract) | US1 T035+ labeling |
| PoC-2 / 5 / 6 | Deferred to US3 / US4 / US6 | Per plan milestones |
