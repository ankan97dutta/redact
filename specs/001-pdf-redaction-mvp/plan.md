# Implementation Plan: Redact Desktop PDF Redaction Secure MVP

**Branch**: `001-pdf-redaction-mvp` | **Date**: 2026-10-30 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-pdf-redaction-mvp/spec.md` plus ratified constitution and user technical direction (Tauri 2, React+TS, Rust, modular domain architecture, local-first).

**Note**: Phase 0 research lives in [research.md](./research.md). Phase 1 design: [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md).

## Summary

Build an open-source, single-user, offline desktop app for Windows and macOS that imports PDFs, runs local detection (native text + OCR), requires explicit review/approval, permanently redacts approved content into **new** files, sanitizes, and verifies with honest `verified | failed | inconclusive` outcomes. UI is React/TypeScript in Tauri 2; all PDF mutation, OCR, detection policy execution, sanitization, and verification run in Rust behind traits. PDF.js (or equivalent) is **render-only**. Permanent redaction uses a mature engine (provisional: `pdf_oxide`) after PoC bake-off — not a from-scratch rewriter and not overlay-as-secure.

## Technical Context

**Language/Version**: Rust (stable, edition 2021+) for backend; TypeScript 5.x + React 18/19 for UI; Tauri 2.x shell

**Primary Dependencies** (provisional — confirm in PoCs):
- Tauri 2, React, Vite
- PDF render: PDF.js (UI only)
- PDF redact/extract/sanitize candidate: `pdf_oxide` (MIT/Apache); compare `pdfrs`
- OCR: platform Vision/WinRT + optional Tesseract
- Serde JSON for rules/reports; tokio for async jobs
- Testing: `cargo test`, Vitest/Playwright (or Tauri WebDriver) for UI gates

**Storage**: Local filesystem only — ephemeral workspace session; persisted rule configuration in app data dir; exports/reports where user chooses

**Testing**: Domain unit tests; engine corpus integration tests; contract tests for Tauri commands; immutability + verification labeling regression suite; CI dependency audit

**Target Platform**: Windows 10/11 and macOS 12+ desktop (Apple Silicon + Intel as feasible)

**Project Type**: Desktop application (Tauri)

**Performance Goals**: Single &lt;20-page text PDF secure path usable in &lt;10 minutes (SC-001); UI remains responsive during OCR/batch (cancellable); bounded concurrency for batch

**Constraints**: Offline core; no auth/cloud; originals immutable; no verified without successful verification; no overlay-as-secure; security &gt; convenience; no forensic overclaims

**Scale/Scope**: Single-user; batches of tens of PDFs typical (not server-scale); secure MVP feature set per spec (page reorder/rotate/delete in; highlights out)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Plan compliance |
|-----------|-----------------|
| I. Privacy & security by design | Local-only pipeline; no document telemetry; honest capability claims |
| II. Secure PDF processing | Permanent engine + sanitize + verify; fail-closed; PDF.js not used for redaction |
| III. Original immutability | Working copies only; export path ≠ source; hash checks in tests |
| IV. Modular engines | Traits: Extract/Ocr/Detect/Redact/Sanitize/Verify; versioned **Capabilities** summary (FR-004a) exposed to UI |
| V. UI / domain / infra split | React presentation; Rust domain crates; adapters in infra |
| VI. Offline-first | No network required for core; packaging embeds WebView2 offline installer option |
| VII. TDD | Domain gates and engine corpus tests first for security paths |
| VIII. Automated security/regression | CI corpus + vuln scan |
| IX. A11y/usability | Keyboard review; non-color status (FR-057–060) |
| X. License governance | Prefer MIT/Apache engines; AGPL MuPDF only with explicit decision |
| XI. Cross-platform | Win + mac first-class; platform OCR behind trait |
| XII. Performance | Bounded workers; cancellation tokens |

**Gate result**: PASS — no unjustified violations. Complexity of multiple engine traits is required by constitution IV/V.

**Post-Phase 1 re-check**: PASS — contracts keep security decisions in Rust domain; UI commands cannot set `verified` directly (see `contracts/tauri-commands.md`).

## Project Structure

### Documentation (this feature)

```text
specs/001-pdf-redaction-mvp/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── tauri-commands.md
│   ├── rule-config.schema.json
│   ├── verification-report.schema.json
│   └── coordinates.md
├── checklists/
└── tasks.md                 # /speckit-tasks (not this command)
```

### Source Code (repository root)

```text
apps/desktop/                 # Tauri 2 application
├── src/                      # React + TypeScript UI
│   ├── features/
│   │   ├── workspace/
│   │   ├── viewer/           # PDF.js render, selection, region draw
│   │   ├── review/           # detection list, approve/reject
│   │   ├── rules/            # rule toggles / custom rules UI
│   │   └── export/           # batch export status / reports
│   ├── shared/ui/
│   └── main.tsx
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs
│       ├── commands/         # thin IPC adapters → domain
│       └── infra/            # filesystem, temp, platform OCR wiring
│
crates/
├── redact-domain/            # pure domain: states, gates, DTOs (no Tauri)
├── redact-detect/            # rule engine + built-in patterns
├── redact-pdf/               # traits + pdf_oxide/pdfrs adapters
├── redact-ocr/               # OcrEngine trait + platform/tesseract adapters
├── redact-verify/            # independent verification
└── redact-test-support/      # corpus helpers, fixtures
│
tests/
├── corpus/                   # PDF fixtures (no real PII; synthetic)
├── integration/
└── contracts/
│
docs/
└── licenses/                 # THIRD_PARTY notices
```

**Structure Decision**: Monorepo with `apps/desktop` (Tauri) and domain/engine crates under `crates/`. Keeps UI unable to link redaction internals except via commands; enables `cargo test -p redact-domain` without WebView.

## Complexity Tracking

> No constitution violations requiring justification. Multi-crate split is mandated by modularity principles, not accidental complexity.

| Item | Why Needed | Simpler Alternative Rejected Because |
|------|------------|-------------------------------------|
| Separate verify crate | Independence from redactor claims | In-process “trust redactor bool” fails PoC-4 / FR-049 |
| Platform OCR + Tesseract | Win/mac quality + CI portability | Single engine either hurts UX or CI |

---

## Architecture and component boundaries

```text
┌─────────────────────────────────────────────────────────────┐
│ React UI (presentation only)                                │
│  Viewer(PDF.js) | Review | Rules | Batch status             │
└─────────────▲───────────────────────────────┬───────────────┘
              │ events/DTOs                   │ Tauri commands
┌─────────────┴───────────────────────────────▼───────────────┐
│ commands/ (infra adapters) — authz-free local IPC            │
└─────────────▲───────────────────────────────┬───────────────┘
              │                               │
┌─────────────┴──────────────┐    ┌───────────▼───────────────┐
│ redact-domain              │    │ Job runtime (tokio)         │
│ lifecycle, approval gates  │◄───│ cancel tokens, workers      │
└─────────────▲──────────────┘    └───────────┬───────────────┘
              │                               │
     ┌────────┴────────┬──────────┬───────────┴──────┬────────┐
     ▼                 ▼          ▼                  ▼        ▼
 ExtractEngine   OcrEngine  RuleEngine   RedactionEngine  VerifyEngine
 (+ Sanitize)         │          │              │              │
                      └──────────┴──────────────┴──────────────┘
                         working copy FS (never original path)
```

**Pipeline stages**: ingest → (unlock) → analyze (extract+OCR) → detect → review/approve → page-ops on working copy → redact approved → sanitize → write export → verify → report.

---

## Technology evaluation (summary)

See [research.md](./research.md) for full trade-offs.

| Area | Choice | Trade-off |
|------|--------|-----------|
| Redaction | `pdf_oxide` provisional | Permissive license + destructive API; must PoC complex fonts/forms |
| Alt redaction | `pdfrs` | Pure Rust; weaker on complex PDFs |
| Avoid | MuPDF AGPL / PDF.js redact | License or false security |
| OCR | Native + Tesseract fallback | Best UX + CI; more adapters |
| Render | PDF.js | Good enough viewer; never mutate |
| Shell | Tauri 2 | Small Rust-native desktop; Win/mac bundles |

---

## Processing lifecycle

1. **Import**: Register source path + content hash; status `imported`.
2. **Unlock** (if encrypted): password in memory → decrypt to working copy → clear password when possible.
3. **Analyze**: Native extract + selective OCR; emit text runs with canonical rects + source tags.
4. **Detect**: RuleEngine over runs → detections `proposed` (never approved).
5. **Review**: User approve/reject/manual regions; bulk select by rule/category/doc.
6. **Page utilities** (optional): reorder/rotate/delete on working copy only.
7. **Export request**: Domain checks approval gate + output path ≠ source + no name collision (block).
8. **Redact + sanitize**: Fail closed → document `failed`/`inconclusive`, never `verified`.
9. **Verify**: Independent checks → report; only then `verified`.
10. **Session end**: Wipe temps; keep persisted rules only.

**Cancellation**: Token checked between stages; cancelled docs not verified.

---

## Security architecture

- Trust boundary: all security-critical decisions in Rust domain/engines.
- UI cannot pass `verificationOutcome: verified` — only server-side verifier sets it.
- Originals: open read-only; exports refuse overwrite of source and existing targets (FR-041, FR-041a).
- Secrets: passwords/session secrets not logged; temp dirs per job with restrictive ACLs.
- Threat model honesty: verification = documented automated checks, not lab forensics.
- Supply chain: license inventory; `cargo audit` / npm audit in CI.

---

## Testing strategy

| Layer | What |
|-------|------|
| Unit | Domain state machine; approval gates; rule enablement; confidence handling |
| Engine corpus | Synthetic PAN/Aadhaar/etc.; image OCR pages; forms; encrypted |
| Immutability | SHA-256 source before/after every job |
| Verification labeling | Incomplete/fail paths never surface as verified |
| Contract | Tauri command JSON schemas / ts-rs or schemars types |
| UI | Keyboard approve/reject; cannot export without approval |
| Cross-platform | CI matrix Win + mac for corpus subset |

---

## Cross-platform considerations

- Path handling via `dunce`/`path` normalization; never assume `/`.
- OCR adapters feature-gated per OS.
- Fonts/raster DPI differences covered in PoC-3.
- Packaging: NSIS + DMG; macOS notarization; Windows signing when certs available.
- Manual release checklist until both CI targets green.

---

## Dependency graph (logical)

```text
apps/desktop
  ├── redact-domain
  ├── commands → redact-domain, redact-detect, redact-pdf, redact-ocr, redact-verify
  └── UI (React) — no direct engine deps

redact-detect → redact-domain
redact-pdf → redact-domain (+ pdf_oxide/pdfrs)
redact-ocr → redact-domain (+ platform/tesseract)
redact-verify → redact-domain, redact-pdf (extract-only APIs)
redact-test-support → fixtures
```

---

## Major technical risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Engine under-redacts complex fonts | False sense of security | Fail-closed + PoC-1 corpus; inconclusive/failed statuses |
| Coord mismatch | Wrong content removed / residue | PoC-3; single canonical space |
| OCR miss on scans | Undetected PII | Honest coverage messaging; manual regions |
| AGPL contamination | License crisis | Ban MuPDF unless AGPL accepted; review transitive deps |
| Temp residue | PII leak on disk | Job-scoped dirs + startup sweep + tests |
| Batch memory blowup | Crash | Bounded workers; streaming page OCR |
| PDF.js mistaken for redaction | Product defect | Architecture + codeowners + tests |

---

## Incremental implementation milestones

Aligned with `tasks.md` PoC placement (not “all PoCs before any story”).

| Milestone | Deliverable | Exit gate / PoC |
|-----------|-------------|-----------------|
| **M0** | Repo scaffold (Tauri app + crates + CI skeleton) | App launches offline shell (Phase 1) |
| **M1** | Blocking PoCs for secure export path | **PoC-1**, **PoC-3**, **PoC-4** recorded; engine selected (Phase 2 T021–T023) |
| **M2** | Working-copy ingest, hash immutability, domain gates | FR-001–003, FR-028a subset (Phase 2–3 foundation) |
| **M3** | US1 secure single-doc redact/sanitize/verify + unsupported-construct labeling + Capabilities domain/API | PoC-1/4; FR-029–034, 032, 043–049, 004/004a |
| **M4** | US7 offline/privacy validation | SC-005 |
| **M5** | US3 OCR on scans/mixed | **PoC-2**; FR-013 |
| **M6** | US2 batch, cancel, retry, export conflicts | FR-035–041a |
| **M7** | US4 encrypted unlock | **PoC-5**; FR-056 |
| **M8** | US5 rule config persist + JSON | FR-014–020, FR-055a |
| **M9** | US6 viewer tools + page ops | **PoC-6**; FR-042–042b |
| **M10** | A11y + packaging + Capabilities UI (SC-013) + corpus CI | FR-057–060, FR-004a/SC-013; constitution VIII/X |

---

## Proof-of-concept checklist (gated by milestone, matching tasks)

| PoC | When (tasks) | Blocks |
|-----|--------------|--------|
| **PoC-1** Redaction engine bake-off | Phase 2 / before US1 redact adapter (T021) | US1 production redaction (T033+) |
| **PoC-3** Coordinate round-trip | Phase 2 (T022) | Viewer↔redact alignment |
| **PoC-4** Independent verification | Phase 2 (T023) | US1 verify/export labeling (T035+) |
| **PoC-2** Local OCR | US3 (T055) | Scan/mixed OCR path |
| **PoC-5** Encrypted unlock | US4 (T074) | Password PDF flow |
| **PoC-6** Page ops immutability | US6 (T089) | Page reorder/rotate/delete |

Do **not** begin US1 production redaction/verify (M3) until **PoC-1** and **PoC-4** pass. Do **not** require PoC-2/5/6 before US1.

---

## Analyze remediation note (C1)

**C1 (FR-004a Capabilities)**: Covered in data-model (`CapabilitiesSummary`), `contracts/tauri-commands.md` (`get_capabilities`), foundational tasks T107–T108, polish tasks T109–T110 (UI + SC-013). Optional G4/G5 polish: T111 (non-goals checklist), T112 (SC-001 timing + SC-007 usability stub).

---

## Next command

Execute `/speckit-implement` against `tasks.md` (includes FR-004a Capabilities tasks).
