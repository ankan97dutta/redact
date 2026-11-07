# Contract: Tauri Commands (IPC)

All commands return `Result&lt;T, AppError&gt;` where `AppError` includes stable `code` for UI.

Security rule: **No command accepts a client-supplied verification outcome.** Only `verify_export` / export pipeline sets report outcome.

## Workspace / documents

### `import_documents`

- **Input**: `{ paths: string[] }`
- **Output**: `{ documents: DocumentSummary[] }`
- **Errors**: `io`, `unsupported`, partial success allowed (per-doc errors in summaries)

### `unlock_document`

- **Input**: `{ document_id: string, password: string }`
- **Output**: `{ status: DocumentStatus }`
- **Notes**: Password not echoed in logs; cleared from command layer after call

### `get_workspace_snapshot`

- **Input**: `{}`
- **Output**: full session DTO (docs, statuses, detections summary)

### `remove_document`

- **Input**: `{ document_id: string }`
- **Side effect**: wipe working copy + secrets for doc

## Analysis / detection

### `analyze_document` / `analyze_batch`

- **Input**: `{ document_ids: string[], cancel_token_id?: string }`
- **Output**: `{ job_id: string }`
- **Events** (Tauri events): `job_progress`, `document_status_changed`

### `get_detections`

- **Input**: `{ document_id: string }`
- **Output**: `{ detections: DetectionDto[], manual_regions: ManualRegionDto[] }`

## Review

### `set_detection_review_state`

- **Input**: `{ ids: string[], state: "approved" | "rejected" | "proposed" }`
- **Output**: `{ updated: number }`

### `add_manual_region`

- **Input**: `{ document_id, page_index, rect: CanonicalRectDto, label?: string }`
- **Output**: `{ region: ManualRegionDto }`  
- **Notes**: `rect` MUST already be canonical user-space (UI converts before invoke)

### `mark_document_approved_for_export`

- **Input**: `{ document_id: string }`
- **Output**: `{ status }` or error `empty_approval_set` / `not_ready`

## Page utilities

### `apply_page_ops`

- **Input**: `{ document_id, ops: PageOpDto[] }`
- **Output**: `{ page_count: number }`
- **Invariant**: `source_hash` unchanged

## Rules

### `get_rule_config` / `update_rule_config` / `import_rule_config_json` / `export_rule_config_json`

- Persist to app data; survive restart

## Export / verify

### `export_documents`

- **Input**: `{ document_ids: string[], output_dir: string }`
- **Behavior**:
  - Skip/not include non-approved
  - Block if output path equals source
  - Block if target filename exists → per-doc `failed` with `file_conflict`
  - Run redact → sanitize → verify internally
- **Output**: `{ job_id: string }`
- **Events**: progress + terminal statuses

### `get_verification_report`

- **Input**: `{ document_id: string }`
- **Output**: `VerificationReportDto`

### `retry_failed_documents`

- **Input**: `{ document_ids: string[], output_dir: string }`
- **Output**: `{ job_id: string }`

## Jobs

### `cancel_job`

- **Input**: `{ job_id: string }`
- **Effect**: cooperative cancel; no doc becomes `verified` due to cancel

## Capabilities (FR-004a)

### `get_capabilities`

- **Input**: `{}`
- **Output**: `CapabilitiesSummaryDto`
- **Notes**: Read-only; assembled from registered engine adapters + verification check registry. MUST include version identifiers and at least one documented limitation. MUST remain consistent with per-document capability warnings and verification labeling (same source of truth as analyze/export gates).

## DTOs (illustrative)

```ts
type CanonicalRectDto = { page_index: number; x0: number; y0: number; x1: number; y1: number };
type DocumentStatus =
  | "imported" | "needs_password" | "analyzing" | "ready_for_review"
  | "reviewing" | "approved_for_export" | "exporting"
  | "verified" | "failed" | "inconclusive" | "unsupported" | "cancelled";

type CapabilityStatus = "supported" | "limited" | "unsupported";
type CapabilitiesSummaryDto = {
  schema_version: string;
  app_version: string;
  engines: { role: string; name: string; version: string }[];
  document_classes: { id: string; status: CapabilityStatus; detail?: string }[];
  verification_checks: { id: string; status: CapabilityStatus; detail?: string }[];
  limitations: string[];
  generated_at: string; // ISO-8601
};
```

OpenAPI-style JSON schemas for reports/rules live alongside this file.