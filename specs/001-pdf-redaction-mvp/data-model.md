# Data Model: Redact Secure MVP

**Feature**: `001-pdf-redaction-mvp`  
**Date**: 2026-10-30

## Entities

### Workspace (session-scoped, ephemeral)

| Field | Type | Notes |
|-------|------|-------|
| id | UUID | Session id |
| created_at | datetime | |
| documents | Document[] | In-memory / session store |
| active_rule_config_id | string | Points at persisted config |
| batch_job | BatchJob? | Current or last job |

**Rules**: Destroyed on app quit; not restored on restart (FR-055b).

### Document

| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| source_path | PathBuf | Original; never written |
| source_hash_sha256 | string | Immutability baseline |
| working_copy_path | PathBuf? | Temp/cache; distinct from source |
| display_name | string | |
| status | DocumentStatus | See state machine |
| page_count | u32? | After open |
| encryption | EncryptionState | `none \| locked \| unlocked` |
| detections | Detection[] | |
| manual_regions | ManualRegion[] | |
| page_ops | PageOp[] | Reorder/rotate/delete intents applied to working copy |
| last_error | string? | User-visible |
| export | ExportRecord? | Last attempt |

### DocumentStatus

```text
imported
needs_password
analyzing
ready_for_review
reviewing
approved_for_export
exporting
verified
failed
inconclusive
unsupported
cancelled
```

**Transitions** (allowed):

| From | To | Guard |
|------|----|-------|
| imported | needs_password | Encrypted |
| imported | analyzing | Readable |
| needs_password | analyzing | Unlock OK |
| needs_password | failed | Unlock give-up / bad file |
| analyzing | ready_for_review | Analyze OK |
| analyzing | unsupported / failed | Analyze error |
| ready_for_review / reviewing | reviewing | User interaction |
| reviewing | approved_for_export | Export eligibility gate |
| approved_for_export | exporting | Export started |
| exporting | verified | Verify success |
| exporting | failed | Redact/sanitize/verify fail or path conflict |
| exporting | inconclusive | Checks incomplete / uncertain residual |
| * | cancelled | Cancel token |

**Forbidden**: Any transition to `verified` except from `exporting` after successful verification report.

### Detection

| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| document_id | UUID | |
| rule_id | string | Built-in or custom |
| category | string | e.g. pan, aadhaar |
| review_state | `proposed \| approved \| rejected` | Never auto-approved |
| confidence | `high \| medium \| low \| uncertain` | Uncertain ≠ confirmed |
| source | `native_text \| ocr` | |
| rect | CanonicalRect | Page user space |
| snippet | string | Redacted in logs if logged at all |
| page_index | u32 | 0-based |

### ManualRegion

| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| document_id | UUID | |
| review_state | same as Detection | Enters as proposed |
| rect | CanonicalRect | From UI transform |
| page_index | u32 | |
| label | string? | Optional user note |

### CanonicalRect

| Field | Type | Notes |
|-------|------|-------|
| page_index | u32 | |
| x0, y0, x1, y1 | f64 | PDF user space, y-up |

Validation: `x1 > x0`, `y1 > y0`; within expanded media box tolerance.

### Rule / RuleConfiguration

| Field | Type | Notes |
|-------|------|-------|
| id | string | Stable id |
| version | semver string | Schema version |
| builtin_toggles | map&lt;rule_id, bool&gt; | Persist across restarts |
| custom_rules | CustomRule[] | |
| updated_at | datetime | |

### CustomRule

| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| name | string | |
| kind | `regex \| keyword \| phrase \| validation` | |
| pattern | string | |
| category | string | |
| enabled | bool | |
| flags | object | casefold, etc. |

### BatchJob

| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| document_ids | UUID[] | |
| phase | `detect \| export` | |
| output_dir | PathBuf? | For export |
| worker_limit | u32 | Default low |
| cancel_requested | bool | |
| per_doc_results | map&lt;UUID, DocumentStatus&gt; | |

### ExportRecord

| Field | Type | Notes |
|-------|------|-------|
| output_path | PathBuf | Must ≠ source; must not exist pre-write (MVP) |
| started_at / finished_at | datetime | |
| verification_report_id | UUID | |

### VerificationReport

| Field | Type | Notes |
|-------|------|-------|
| id | UUID | |
| document_id | UUID | |
| outcome | `verified \| failed \| inconclusive` | |
| checks | CheckResult[] | |
| engine_versions | map | Redactor/verifier ids |
| created_at | datetime | |

### CheckResult

| Field | Type | Notes |
|-------|------|-------|
| name | string | e.g. `text_residue` |
| status | `pass \| fail \| skip \| error` | |
| detail | string | |

**Aggregation**: Any `fail`/`error` ⇒ cannot be `verified`. Any required check `skip`/`error` ⇒ `inconclusive` or `failed` per policy (MVP: required skips ⇒ `inconclusive`).

### PageOp

| Field | Type | Notes |
|-------|------|-------|
| kind | `reorder \| rotate \| delete` | |
| payload | JSON | indices / degrees |

Applied only to working copy before redact.

### Secrets

Password material is **not** a durable entity. Held in process memory keyed by `document_id` until unlock completes or document removed.

## Export eligibility (domain gate)

Document may enter `approved_for_export` / be included in export job iff:

1. Status is `reviewing` or `ready_for_review` with explicit user “mark approved for export”
2. At least one approved Detection or ManualRegion (empty set blocked per MVP assumption)
3. No unresolved `needs_password`
4. Not `unsupported` / `failed` terminal without retry

## Relationships

```text
Workspace 1──* Document
Document 1──* Detection
Document 1──* ManualRegion
Document 1──0..1 ExportRecord
ExportRecord 1──1 VerificationReport
RuleConfiguration 1──* CustomRule
BatchJob *──* Document (by id)
```
