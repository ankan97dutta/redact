# Tasks: Redact Desktop PDF Redaction Secure MVP

**Input**: Design documents from `/specs/001-pdf-redaction-mvp/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/, quickstart.md, constitution.md

**Tests**: Included — constitution Principle VII (TDD) is non-negotiable for domain gates, approval, export eligibility, sanitization, and verification labeling. Write failing tests before implementation in those areas.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: User story label (US1–US7) for story phases only
- Descriptions include exact file paths

## Path Conventions

Per plan.md: `apps/desktop/` (Tauri + React), `crates/redact-*`, `tests/corpus|integration|contracts`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Monorepo and toolchains so crates and the Tauri app can build

- [ ] T001 Create monorepo roots `apps/desktop/`, `crates/`, `tests/corpus/`, `tests/integration/`, `tests/contracts/`, `docs/licenses/` per `specs/001-pdf-redaction-mvp/plan.md`
- [ ] T002 [P] Initialize Cargo workspace in root `Cargo.toml` listing `crates/redact-domain`, `crates/redact-detect`, `crates/redact-pdf`, `crates/redact-ocr`, `crates/redact-verify`, `crates/redact-test-support`, `apps/desktop/src-tauri`
- [ ] T003 [P] Scaffold Tauri 2 + React + TypeScript app under `apps/desktop/` (`package.json`, `vite.config.ts`, `src/main.tsx`, `src-tauri/tauri.conf.json`)
- [ ] T004 [P] Add workspace lint/format config (root `rustfmt.toml`, `clippy` workspace flags, `apps/desktop/eslint.config.js` or project equivalent)
- [ ] T005 [P] Add CI skeleton `.github/workflows/ci.yml` with `cargo test`, frontend typecheck, and dependency audit placeholders
- [ ] T006 [P] Create `docs/licenses/THIRD_PARTY.md` stub and note engine license evaluation from `specs/001-pdf-redaction-mvp/research.md`
- [ ] T007 [P] Add synthetic corpus placeholders `tests/corpus/README.md` documenting no-real-PII policy

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Domain types, engine traits, temp/password infra, PoC gates, and IPC shell — MUST complete before user story UI/flows

**⚠️ CRITICAL**: No user story implementation until this phase completes (PoC-1 and PoC-4 must pass before production redaction path work in US1)

- [ ] T008 Implement domain DTOs and enums (`DocumentStatus` including `imported|needs_password|analyzing|ready_for_review|reviewing|approved_for_export|exporting|verified|failed|inconclusive|unsupported|cancelled`, confidence `high|medium|low|uncertain`, review_state `proposed|approved|rejected`) in `crates/redact-domain/src/types.rs`
- [ ] T009 [P] Implement `CanonicalRect` with validation `x1 > x0`, `y1 > y0` in `crates/redact-domain/src/geometry.rs` per `specs/001-pdf-redaction-mvp/contracts/coordinates.md`
- [ ] T010 [P] Implement document lifecycle transition guards forbidding client-set `verified` except `exporting → verified` after verify success in `crates/redact-domain/src/lifecycle.rs`
- [ ] T011 [P] Implement export eligibility gate (non-empty approval set; block empty export) in `crates/redact-domain/src/gates.rs`
- [ ] T012 Write failing unit tests for lifecycle + eligibility in `crates/redact-domain/tests/lifecycle_gates.rs` (TDD)
- [ ] T013 [P] Define `ExtractionEngine`, `RedactionEngine`, `SanitizeEngine` traits in `crates/redact-pdf/src/traits.rs`
- [ ] T014 [P] Define `OcrEngine` trait returning text + boxes in `crates/redact-ocr/src/traits.rs`
- [ ] T015 [P] Define `RuleEngine` trait in `crates/redact-detect/src/traits.rs`
- [ ] T016 [P] Define `VerificationEngine` trait that alone produces `VerificationReport.outcome` in `crates/redact-verify/src/traits.rs`
- [ ] T017 Implement job-scoped temp working-copy manager with restrictive permissions and startup orphan sweep in `apps/desktop/src-tauri/src/infra/temp_files.rs`
- [ ] T018 [P] Implement in-memory password/secret holder (no log/report persistence) in `apps/desktop/src-tauri/src/infra/secrets.rs`
- [ ] T019 [P] Implement coordinate transform helpers `render_css_to_user` / `user_to_render_css` / `raster_px_to_user` in `crates/redact-domain/src/coords.rs`
- [ ] T020 Write failing coordinate round-trip tests (≤2 pt tolerance) in `crates/redact-domain/tests/coords_roundtrip.rs`
- [ ] T021 Execute **PoC-1** redaction engine bake-off; record results in `specs/001-pdf-redaction-mvp/poc-results.md`; select primary adapter under `crates/redact-pdf/src/adapters/`
- [ ] T022 [P] Execute **PoC-3** coordinate round-trip against viewer fixture; append results to `specs/001-pdf-redaction-mvp/poc-results.md`
- [ ] T023 [P] Execute **PoC-4** independent verification catches residual text; append results to `specs/001-pdf-redaction-mvp/poc-results.md`
- [ ] T024 Wire thin Tauri command module skeleton (no security outcomes from UI) in `apps/desktop/src-tauri/src/commands/mod.rs` per `specs/001-pdf-redaction-mvp/contracts/tauri-commands.md`
- [ ] T025 [P] Add shared `AppError` with stable `code` field in `crates/redact-domain/src/error.rs`
- [ ] T026 [P] Add `redact-test-support` corpus helpers in `crates/redact-test-support/src/lib.rs`
- [ ] T027 Implement session `Workspace` store (ephemeral; not restored on restart) in `crates/redact-domain/src/workspace.rs`

**Checkpoint**: Foundation ready — user stories can begin; PoC-1 and PoC-4 recorded as pass before US1 export path

---

## Phase 3: User Story 1 - Secure single-document redaction with review (Priority: P1) 🎯 MVP

**Goal**: Import one PDF, detect with built-ins, review/approve/reject, export new file with sanitize+verify, never mark incomplete verification as verified, never modify original

**Independent Test**: Import text PDF with known sensitive strings → approve → export → source hash unchanged → targets not extractable → report status matches verification (quickstart Scenario A)

### Tests for User Story 1

- [ ] T028 [P] [US1] Write failing domain tests that incomplete/failed verification cannot yield `verified` in `crates/redact-domain/tests/verification_labeling.rs`
- [ ] T029 [P] [US1] Write failing immutability test (source SHA-256 unchanged after job) in `tests/integration/immutability_us1.rs`
- [ ] T030 [P] [US1] Write failing contract tests for `import_documents`, `set_detection_review_state`, `export_documents`, `get_verification_report` in `tests/contracts/tauri_us1.rs`

### Implementation for User Story 1

- [ ] T031 [P] [US1] Implement `Document`, `Detection`, `ExportRecord`, `VerificationReport`, `CheckResult` models in `crates/redact-domain/src/entities.rs` per data-model field constraints
- [ ] T032 [P] [US1] Implement pdf extract adapter (native text + quads → `CanonicalRect`) in `crates/redact-pdf/src/adapters/extract.rs`
- [ ] T033 [US1] Implement selected redaction adapter (fail-closed) in `crates/redact-pdf/src/adapters/redact.rs` (depends on T021)
- [ ] T034 [P] [US1] Implement sanitize adapter (metadata/JS/embedded policy toggles) in `crates/redact-pdf/src/adapters/sanitize.rs`
- [ ] T035 [US1] Implement independent verifier (re-open export bytes; text residue checks) in `crates/redact-verify/src/engine.rs` (depends on T023)
- [ ] T036 [P] [US1] Implement MVP built-in detectors (PAN, Aadhaar, GSTIN, IFSC, bank, phone, email baseline) emitting confidence/source in `crates/redact-detect/src/builtins/`
- [ ] T037 [US1] Implement analyze→detect pipeline orchestration in `crates/redact-domain/src/pipeline/analyze.rs`
- [ ] T038 [US1] Implement export pipeline order redact→sanitize→write→verify with outcome aggregation (`fail`/`error` ⇒ not verified; required `skip` ⇒ `inconclusive`/`failed`) in `crates/redact-domain/src/pipeline/export.rs`
- [ ] T039 [US1] Implement Tauri commands `import_documents`, `analyze_document`, `get_detections`, `set_detection_review_state`, `mark_document_approved_for_export`, `export_documents`, `get_verification_report` in `apps/desktop/src-tauri/src/commands/documents.rs`
- [ ] T040 [P] [US1] Build workspace import + status UI in `apps/desktop/src/features/workspace/`
- [ ] T041 [P] [US1] Build PDF.js render-only viewer with proposal overlays (no mutation) in `apps/desktop/src/features/viewer/`
- [ ] T042 [US1] Build review list (approve/reject, navigate to page/region) in `apps/desktop/src/features/review/` (depends on T041)
- [ ] T043 [US1] Build export + verification report UI distinguishing verified|failed|inconclusive in `apps/desktop/src/features/export/`
- [ ] T044 [US1] Enforce UI cannot set verification outcome (types omit writable outcome) in `apps/desktop/src/shared/types/ipc.ts`
- [ ] T045 [US1] Add export path ≠ source guard and user confirmation for permanent export in `crates/redact-domain/src/gates.rs` and `apps/desktop/src/features/export/ExportDialog.tsx`
- [ ] T046 [US1] Generate synthetic `tests/corpus/text_pan_sample.pdf` via `crates/redact-test-support` and wire quickstart Scenario A automation in `tests/integration/quickstart_scenario_a.rs`

**Checkpoint**: US1 MVP secure path works independently

---

## Phase 4: User Story 7 - Offline privacy guarantees (Priority: P1)

**Goal**: Core path works with network disabled; no document telemetry; temps cleaned; no silent long-term PII store

**Independent Test**: Airplane mode → full US1 path succeeds; no content telemetry; temps removed after job (quickstart Scenario B)

### Tests for User Story 7

- [ ] T047 [P] [US7] Write failing test that export pipeline functions with network-denied environment flag in `tests/integration/offline_us7.rs`
- [ ] T048 [P] [US7] Write failing test that temp working copies are removed after success/cancel in `apps/desktop/src-tauri/src/infra/temp_files_tests.rs`

### Implementation for User Story 7

- [ ] T049 [P] [US7] Audit dependencies for outbound network; document in `docs/licenses/NETWORK.md` (none required for core)
- [ ] T050 [US7] Ensure no analytics/telemetry hooks send document content or paths in `apps/desktop/src/` and `apps/desktop/src-tauri/`
- [ ] T051 [US7] Hook job completion/cancel/quit to temp cleanup in `apps/desktop/src-tauri/src/infra/temp_files.rs`
- [ ] T052 [US7] Add offline validation script/checklist step referencing quickstart Scenario B in `specs/001-pdf-redaction-mvp/quickstart.md` (keep in sync)

**Checkpoint**: Offline + privacy guarantees validated on US1 path

---

## Phase 5: User Story 3 - Scanned and mixed PDF OCR (Priority: P1)

**Goal**: Local OCR on scans/mixed PDFs; detections tagged `ocr` vs `native_text`; honest limited-coverage messaging

**Independent Test**: Import scan fixture → OCR detections → approve/export with honest verification (quickstart Scenario D); **PoC-2** must pass

### Tests for User Story 3

- [ ] T053 [P] [US3] Write failing OCR coordinate mapping tests in `crates/redact-ocr/tests/raster_to_user.rs`
- [ ] T054 [P] [US3] Write failing integration test for scan corpus detection source=`ocr` in `tests/integration/ocr_us3.rs`

### Implementation for User Story 3

- [ ] T055 [US3] Execute **PoC-2** local OCR; record in `specs/001-pdf-redaction-mvp/poc-results.md`
- [ ] T056 [P] [US3] Implement macOS Vision OCR adapter in `crates/redact-ocr/src/adapters/macos_vision.rs`
- [ ] T057 [P] [US3] Implement Windows WinRT OCR adapter in `crates/redact-ocr/src/adapters/windows_ocr.rs`
- [ ] T058 [P] [US3] Implement Tesseract fallback adapter in `crates/redact-ocr/src/adapters/tesseract.rs`
- [ ] T059 [US3] Integrate OCR into analyze pipeline for image/scanned pages with coverage warnings in `crates/redact-domain/src/pipeline/analyze.rs`
- [ ] T060 [US3] Surface detection `source` and limited-coverage warnings in `apps/desktop/src/features/review/`
- [ ] T061 [US3] Add `tests/corpus/scan_gstin_sample.pdf` synthetic fixture via `crates/redact-test-support`

**Checkpoint**: Scanned/mixed PDFs independently testable

---

## Phase 6: User Story 2 - CA batch redaction (Priority: P1)

**Goal**: Multi-doc workspace, shared rules, bulk select, approved-only export, per-doc status/errors, retry, output dir, block existing filename conflicts

**Independent Test**: 3 PDFs → approve 2 → conflict on one name → per-doc statuses; originals untouched (quickstart Scenario C)

### Tests for User Story 2

- [ ] T062 [P] [US2] Write failing tests for file_conflict block (no overwrite, continue others) in `crates/redact-domain/tests/export_conflicts.rs`
- [ ] T063 [P] [US2] Write failing batch cancel token tests (cancelled ≠ verified) in `crates/redact-domain/tests/batch_cancel.rs`
- [ ] T064 [P] [US2] Write failing contract tests for `analyze_batch`, `export_documents`, `retry_failed_documents`, `cancel_job` in `tests/contracts/tauri_us2.rs`

### Implementation for User Story 2

- [ ] T065 [P] [US2] Implement `BatchJob` entity and bounded worker pool with `CancellationToken` in `crates/redact-domain/src/batch.rs`
- [ ] T066 [US2] Implement bulk selection helpers (by rule/category across docs) in `crates/redact-domain/src/selection.rs`
- [ ] T067 [US2] Implement export conflict policy FR-041a in `crates/redact-domain/src/pipeline/export.rs`
- [ ] T068 [US2] Implement Tauri commands `analyze_batch`, `retry_failed_documents`, `cancel_job` + progress events in `apps/desktop/src-tauri/src/commands/batch.rs`
- [ ] T069 [P] [US2] Build batch status/error/retry UI in `apps/desktop/src/features/workspace/BatchPanel.tsx`
- [ ] T070 [US2] Extend review UI for cross-document bulk select in `apps/desktop/src/features/review/BulkSelect.tsx`
- [ ] T071 [US2] Output directory picker + conflict messaging in `apps/desktop/src/features/export/BatchExportDialog.tsx`

**Checkpoint**: Batch CA workflow independently testable

---

## Phase 7: User Story 4 - Encrypted PDF unlock (Priority: P2)

**Goal**: Prompt for password, clear errors on wrong password, corrupted/unsupported statuses without crashing workspace

**Independent Test**: Wrong password → error/retry; correct → analyze; corrupted → failed/unsupported (quickstart Scenario E); **PoC-5**

### Tests for User Story 4

- [ ] T072 [P] [US4] Write failing unlock/wrong-password tests in `tests/integration/encrypted_us4.rs`
- [ ] T073 [P] [US4] Write failing test that passwords never appear in verification report JSON in `crates/redact-verify/tests/no_secrets_in_report.rs`

### Implementation for User Story 4

- [ ] T074 [US4] Execute **PoC-5**; record in `specs/001-pdf-redaction-mvp/poc-results.md`
- [ ] T075 [US4] Implement unlock→working-copy decrypt path using secrets holder in `crates/redact-pdf/src/adapters/unlock.rs` and `apps/desktop/src-tauri/src/commands/documents.rs` (`unlock_document`)
- [ ] T076 [P] [US4] Map corrupted/unsupported errors to `DocumentStatus` + user-visible reasons in `crates/redact-domain/src/error.rs`
- [ ] T077 [US4] Password prompt UI with retry in `apps/desktop/src/features/workspace/UnlockDialog.tsx`
- [ ] T078 [US4] Ensure batch continues when one doc is unsupported in `crates/redact-domain/src/batch.rs`

**Checkpoint**: Encrypted/malformed handling independently testable

---

## Phase 8: User Story 5 - Configurable detection rules (Priority: P2)

**Goal**: Toggle built-ins; custom regex/keyword/phrase/validation; JSON import/export; persist rule config across restarts (not documents)

**Independent Test**: Disable rule → no new matches; custom keyword matches; JSON round-trip; restart keeps toggles (quickstart Scenario F / SC-010/011)

### Tests for User Story 5

- [ ] T079 [P] [US5] Write failing JSON schema validation tests against `specs/001-pdf-redaction-mvp/contracts/rule-config.schema.json` in `crates/redact-detect/tests/rule_config_schema.rs`
- [ ] T080 [P] [US5] Write failing persistence tests (rules survive restart; workspace docs do not) in `tests/integration/rules_persist_us5.rs`

### Implementation for User Story 5

- [ ] T081 [P] [US5] Implement `RuleConfiguration` / `CustomRule` models and serde in `crates/redact-detect/src/config.rs`
- [ ] T082 [P] [US5] Complete remaining built-ins (addresses, names, case references, financial identifiers) as best-effort/`uncertain`-capable in `crates/redact-detect/src/builtins/`
- [ ] T083 [US5] Implement custom rule evaluation (regex/keyword/phrase/validation) in `crates/redact-detect/src/custom.rs`
- [ ] T084 [US5] Persist rule config in app data dir (survive restart) in `apps/desktop/src-tauri/src/infra/rule_store.rs`
- [ ] T085 [US5] Implement Tauri commands `get_rule_config`, `update_rule_config`, `import_rule_config_json`, `export_rule_config_json` in `apps/desktop/src-tauri/src/commands/rules.rs`
- [ ] T086 [P] [US5] Build rules UI (toggles, custom editor, import/export) in `apps/desktop/src/features/rules/`

**Checkpoint**: Rule configuration independently testable

---

## Phase 9: User Story 6 - Manual regions, viewer tools, page utilities (Priority: P2)

**Goal**: Zoom/view-rotate/navigate, text select, draw rectangles, multi-region preview; page reorder/rotate/delete on working copy only; no highlights/annotations in MVP

**Independent Test**: Manual region-only export; page delete reduces export pages; source hash unchanged (quickstart Scenario G); **PoC-6**

### Tests for User Story 6

- [ ] T087 [P] [US6] Write failing tests that page ops do not change `source_hash_sha256` in `tests/integration/page_ops_us6.rs`
- [ ] T088 [P] [US6] Write failing contract tests for `add_manual_region` and `apply_page_ops` in `tests/contracts/tauri_us6.rs`

### Implementation for User Story 6

- [ ] T089 [US6] Execute **PoC-6**; record in `specs/001-pdf-redaction-mvp/poc-results.md`
- [ ] T090 [P] [US6] Extend viewer: zoom, view rotate, page nav, text select in `apps/desktop/src/features/viewer/`
- [ ] T091 [US6] Implement rectangle draw → canonical rect → `add_manual_region` command in `apps/desktop/src/features/viewer/RegionDraw.tsx` and `apps/desktop/src-tauri/src/commands/documents.rs`
- [ ] T092 [US6] Implement page reorder/rotate/delete on working copy in `crates/redact-pdf/src/adapters/page_ops.rs`
- [ ] T093 [US6] Wire `apply_page_ops` Tauri command + UI in `apps/desktop/src/features/workspace/PageOpsBar.tsx`
- [ ] T094 [US6] Ensure proposals visually distinct and no highlight/annotation tools ship (guard/docs) in `apps/desktop/src/features/viewer/README.md`

**Checkpoint**: Viewer + page utilities independently testable

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Packaging, a11y, CI corpus, hardening across stories

- [ ] T095 [P] Keyboard operability for approve/reject/export confirm in `apps/desktop/src/features/review/` and `apps/desktop/src/features/export/` (FR-057)
- [ ] T096 [P] Non-color-only status labels and focus order pass in `apps/desktop/src/shared/ui/StatusBadge.tsx`
- [ ] T097 Configure Windows NSIS + macOS DMG bundling (offline WebView2 installer option) in `apps/desktop/src-tauri/tauri.conf.json`
- [ ] T098 [P] Expand CI matrix for Windows + macOS corpus subset in `.github/workflows/ci.yml`
- [ ] T099 [P] Run `cargo audit` / npm audit in CI; refresh `docs/licenses/THIRD_PARTY.md`
- [ ] T100 Execute full quickstart scenarios A–G; record sign-off in `specs/001-pdf-redaction-mvp/quickstart-results.md`
- [ ] T101 [P] Performance: confirm bounded workers + cancel under large batch in `crates/redact-domain/tests/batch_load.rs`
- [ ] T102 Codeowners/docs note: PDF.js is render-only; redaction only via Rust engines in `docs/ARCHITECTURE.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 Setup**: Start immediately
- **Phase 2 Foundational**: After Setup — **blocks all stories**; includes PoC-1/3/4
- **Phase 3 US1**: After Foundational — **MVP**
- **Phase 4 US7**: After US1 (validates privacy on secure path)
- **Phase 5 US3**: After Foundational; integrates with analyze used by US1 (PoC-2)
- **Phase 6 US2**: After US1 (reuses export/review); can parallelize with US3 after US1 if staffed carefully
- **Phase 7–9 (P2)**: After Foundational; ideally after US1 MVP validated
- **Phase 10 Polish**: After target stories complete

### User Story Dependencies

| Story | Priority | Depends on | Independently testable? |
|-------|----------|------------|-------------------------|
| US1 | P1 | Foundational | Yes — MVP |
| US7 | P1 | US1 path | Yes — offline/privacy checks |
| US3 | P1 | Foundational (+ analyze hooks) | Yes — scan corpus |
| US2 | P1 | US1 export/review primitives | Yes — batch corpus |
| US4 | P2 | Foundational secrets/temp | Yes — encrypted corpus |
| US5 | P2 | Detect crate | Yes — rule JSON/persist |
| US6 | P2 | Viewer + working copy | Yes — manual/page-ops |

### Parallel Opportunities

- Phase 1: T002–T007 marked [P]
- Phase 2: trait definitions T013–T016, infra T017–T018, coords T019 in parallel after T008
- After Foundational: US3 and US5 can proceed in parallel with different owners while US1 finishes UI
- Within stories: [P] test tasks and parallel model/adapter tasks as marked

---

## Parallel Example: User Story 1

```bash
# After T012 lifecycle tests exist, launch US1 failing tests together:
Task: T028 verification_labeling.rs
Task: T029 immutability_us1.rs
Task: T030 tauri_us1.rs

# Parallel adapters (different files) after PoC-1:
Task: T032 extract.rs
Task: T034 sanitize.rs
Task: T036 builtins/
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 Setup  
2. Complete Phase 2 Foundational (**PoC-1 + PoC-4 required**)  
3. Complete Phase 3 US1  
4. **STOP and VALIDATE** quickstart Scenario A  
5. Demo secure single-doc redaction  

### Incremental Delivery

1. US1 → secure MVP  
2. US7 → prove offline/privacy  
3. US3 → scans  
4. US2 → batch CA workflows  
5. US4 → encryption  
6. US5 → rule packs  
7. US6 → manual regions + page ops  
8. Polish → package + a11y + CI  

### Suggested MVP Scope

**Phases 1–3 (Setup + Foundational + US1)** — optionally include Phase 4 (US7) before any external demo claiming privacy.

---

## Notes

- Do not claim forensic-grade guarantees beyond documented verification checks  
- Do not use PDF.js for permanent redaction  
- Do not implement PDF content removal from scratch unless PoC-1 rejects all libraries with recorded justification  
- Prefer MIT/Apache engines; AGPL MuPDF only with explicit license decision  
- Commit after each task or logical group; use `/speckit-git-commit` hooks as desired  
