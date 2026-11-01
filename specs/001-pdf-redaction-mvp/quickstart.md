# Quickstart Validation Guide: Redact Secure MVP

**Feature**: `001-pdf-redaction-mvp`  
**Purpose**: Runnable validation scenarios proving the secure path end-to-end. Not an implementation guide.

## Prerequisites

- macOS 12+ or Windows 10/11
- Rust stable + Node (versions pinned in repo when scaffold exists)
- Network **disabled** for core scenario validation (airplane mode / firewall)
- Synthetic corpus under `tests/corpus/` (no real client PII)

## Setup (after M0 scaffold)

```bash
# From repo root
pnpm install
cargo build -p redact-domain
pnpm --filter desktop tauri dev
```

Packaged builds (M9+):

```bash
pnpm --filter desktop tauri build
# Windows: NSIS under target/.../bundle/nsis/
# macOS: DMG under target/.../bundle/dmg/
```

## Scenario A — Single-document secure path (P1)

1. Import `tests/corpus/text_pan_sample.pdf`.
2. Confirm status reaches `ready_for_review` with PAN detections `proposed`.
3. Reject one false positive; approve remaining; add one manual region if needed.
4. Record `sha256` of source.
5. Export to empty output folder.
6. **Expect**: new file created; source hash unchanged; report `verified` only if checks pass; approved PAN strings not extractable from export via app verify/extract tooling.
7. Repeat with forced verifier failure fixture → **Expect**: `failed` or `inconclusive`, never `verified`.

## Scenario B — Offline

1. Disable network.
2. Repeat Scenario A.
3. **Expect**: success without network errors.

## Scenario C — Batch + conflict

1. Import 3 corpus PDFs; analyze; approve 2.
2. Pre-create an output filename matching one approved doc’s default export name.
3. Export approved set.
4. **Expect**: conflicting doc `failed`/`file_conflict`; other approved doc may `verified`; unapproved not exported; originals unchanged.

## Scenario D — Scanned OCR

1. Import `tests/corpus/scan_gstin_sample.pdf`.
2. **Expect**: detections with `source=ocr`; coverage warning if OCR weak.
3. Approve and export.
4. **Expect**: honest verification outcome; no overlay-only success.

## Scenario E — Encrypted

1. Import password PDF with wrong password → clear error, retry.
2. Unlock with correct password → analyze proceeds.
3. **Expect**: password not present in verification report JSON.

## Scenario F — Persistence split

1. Toggle a built-in rule off; quit app; relaunch.
2. **Expect**: rule still off.
3. **Expect**: previous documents/detections **not** restored.

## Scenario G — Page ops immutability

1. Import multi-page PDF; delete last page in UI (working copy).
2. **Expect**: source hash unchanged; export page count reduced.

## PoC validation (before M6)

Follow gates in [research.md](./research.md) PoC-1…PoC-6; record results in `specs/001-pdf-redaction-mvp/poc-results.md` (create when executing).

## References

- Domain states: [data-model.md](./data-model.md)
- IPC: [contracts/tauri-commands.md](./contracts/tauri-commands.md)
- Coords: [contracts/coordinates.md](./contracts/coordinates.md)
- Spec acceptance: [spec.md](./spec.md)
