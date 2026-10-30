# Feature Specification: Redact Desktop PDF Redaction (Secure MVP)

**Feature Branch**: `001-pdf-redaction-mvp`

**Created**: 2026-10-30

**Status**: Draft

**Input**: User description: "Complete product specification for Redact, an open-source, local-first desktop PDF redaction application for Windows and macOS targeting chartered accountants, lawyers, and other professionals handling confidential documents."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Secure single-document redaction with review (Priority: P1)

A lawyer opens a confidential PDF, sees proposed sensitive detections, reviews each item on the exact page, rejects false positives, adds any missing manual regions, approves the set, exports a new redacted file, and receives a verification report that distinguishes verified success from failure or inconclusive results—without the original file being changed.

**Why this priority**: This is the minimum secure value of the product. Without permanent redaction, explicit approval, original preservation, and honest verification, the application must not claim to redact.

**Independent Test**: Import one text PDF with known sensitive strings, approve detections, export, confirm original unchanged, confirm redacted strings are not extractable from the export, and confirm the report status matches verification outcome.

**Acceptance Scenarios**:

1. **Given** a readable text PDF is imported, **When** detection runs with built-in rules enabled, **Then** proposed detections appear with category, location, confidence/source cues, and none are treated as user-approved until the user acts.
2. **Given** detections are listed, **When** the user selects a detection, **Then** the viewer navigates to that page and highlights the proposed region.
3. **Given** the user rejects a false positive and approves remaining items (and any manual regions), **When** they request export, **Then** the app creates a new output file, leaves the original unmodified, and runs sanitization then verification before reporting outcome.
4. **Given** verification fails or is incomplete, **When** the export finishes processing, **Then** the result is not marked successful/verified, and the user sees a failed or inconclusive outcome with explanation.

---

### User Story 2 - CA batch redaction across multiple client PDFs (Priority: P1)

A chartered accountant imports multiple client PDFs into one workspace, runs the same detection rule set across the batch, reviews findings (including bulk select by rule/category), approves documents ready for redaction, exports to a chosen output folder, and sees per-document status with retry for failures—without any original being overwritten.

**Why this priority**: Professional CA workflows are batch-heavy; secure multi-document handling is core MVP value alongside single-document flow.

**Independent Test**: Import three PDFs (mix of clean and sensitive content), run detection, approve two, leave one unapproved, export approved only, confirm per-document statuses and that originals are untouched.

**Acceptance Scenarios**:

1. **Given** multiple PDFs are imported, **When** the user applies common detection rules, **Then** each document shows processing status and its own detection list.
2. **Given** detections across documents, **When** the user selects all occurrences of a category (e.g., PAN), **Then** matching detections across the batch are selected for review actions.
3. **Given** some documents are fully reviewed and approved and others are not, **When** the user starts batch export, **Then** only approved documents are processed for permanent redaction export.
4. **Given** one document fails verification, **When** the batch completes, **Then** that document is marked failed (not verified), others can still be verified independently, and the user can retry the failed document.

---

### User Story 3 - Scanned and mixed PDF handling with local OCR (Priority: P1)

A professional imports a scanned or mixed PDF. The app performs local text extraction and/or local OCR as needed, proposes detections on recognized text, and allows the same review → approve → permanent redact → verify path.

**Why this priority**: Many legal and CA packets are scans; excluding OCR would leave a large share of real documents unprotected.

**Independent Test**: Import a scanned PDF containing a known sensitive identifier rendered as image text; after local OCR and detection, approve and export; confirm the identifier is not recoverable via extraction from the export where the engine claims coverage, or else the result is not marked verified.

**Acceptance Scenarios**:

1. **Given** a scanned PDF is imported, **When** analysis runs offline, **Then** OCR proceeds locally and the user can see that detections came from OCR vs native text.
2. **Given** a mixed PDF (native text + image pages), **When** analysis completes, **Then** detections may come from both sources and each detection shows its source.
3. **Given** OCR cannot produce usable text for a page, **When** analysis finishes, **Then** the user is informed that automated detection coverage is limited for that page and is not told that the page was fully scanned for all rule types.

---

### User Story 4 - Encrypted PDF unlock and graceful failure (Priority: P2)

A user imports a password-protected PDF, supplies the password when prompted, and continues the normal workflow. Invalid passwords, corrupted files, and unsupported constructs fail clearly without crashing or silently skipping security steps.

**Why this priority**: Encrypted packets are common in professional practice; graceful failure is required for trust, but core redaction path remains P1.

**Independent Test**: Attempt import with wrong password, then correct password; attempt a corrupted file; confirm clear errors and no partial “verified” export.

**Acceptance Scenarios**:

1. **Given** an encrypted PDF, **When** imported without a password, **Then** the app prompts for credentials and does not claim the document is ready for redaction until unlock succeeds.
2. **Given** an incorrect password, **When** unlock is attempted, **Then** the user sees a clear error and can retry; no silent empty workspace entry pretending success.
3. **Given** a corrupted or unsupported PDF, **When** import or processing fails, **Then** the document status is failed/unsupported with a human-readable reason and other batch items continue independently.

---

### User Story 5 - Configurable detection rules and portable rule packs (Priority: P2)

A firm customizes which built-in patterns run (PAN, Aadhaar, GSTIN, IFSC, bank accounts, phones, emails, addresses, names, case references, financial identifiers), adds custom regex/keywords/phrases/validation rules, toggles rules, and imports/exports rule configuration as JSON for reuse.

**Why this priority**: Built-in India-centric professional patterns are MVP-critical; deep custom rule authoring and pack portability strongly support firms but can follow the first secure path if capacity is constrained—still required for the stated product, prioritized P2 relative to secure export.

**Independent Test**: Disable one built-in rule and confirm it no longer proposes matches; add a custom keyword rule; export JSON; import on a clean profile and see the same toggles/rules.

**Acceptance Scenarios**:

1. **Given** built-in rules exist, **When** the user disables a rule, **Then** subsequent detection runs omit that rule’s new proposals.
2. **Given** the user adds a custom regex or keyword rule, **When** detection runs, **Then** matches appear under that rule with confidence/source metadata and are not auto-approved.
3. **Given** a rule configuration, **When** the user exports JSON and later imports it, **Then** enabled/disabled state and custom rules are restored without requiring network access.

---

### User Story 6 - Manual regions, viewer tools, and page utilities (Priority: P2)

A user navigates pages, zooms/rotates the view, selects text, draws rectangular redaction regions, sees proposed redactions before application, and can reorder, rotate, or delete pages in the working document copy used for export preparation—without highlight/annotation tools in MVP.

**Why this priority**: Manual capture closes gaps automation misses; viewer basics are required to review; page reorder/rotate/delete support packet prep for CA/legal workflows. Highlights/annotations are deferred.

**Independent Test**: Draw a rectangle over sensitive content with no auto-detection, approve only that region, export, and confirm content under the region is not extractable / verification status is honest. Separately, reorder or delete a page on a working copy and confirm the original source file is unchanged.

**Acceptance Scenarios**:

1. **Given** a document is open, **When** the user zooms, rotates the view, and changes pages, **Then** the document remains navigable and proposed regions stay correctly anchored to page locations.
2. **Given** the user draws a rectangular region, **When** they mark it for redaction, **Then** it appears in the review list as a manual item requiring approval like detections.
3. **Given** proposed redactions exist, **When** viewing before export, **Then** proposals are visible as proposals (not yet permanent) and are not presented as annotations or highlights.
4. **Given** a multi-page document in the workspace, **When** the user reorders, rotates, or deletes pages, **Then** those edits apply only to the working/export preparation path and the original source file remains unmodified.

---

### User Story 7 - Offline privacy guarantees for professionals (Priority: P1)

A user completes detection, review, redaction, and verification with network disabled. No document contents are sent externally; temporary materials are handled securely and not silently retained as a hidden archive of client data.

**Why this priority**: Non-negotiable for the product’s professional trust model and constitution.

**Independent Test**: Disable network, run a full single-document secure path end-to-end; confirm success without network; confirm no document telemetry events containing contents.

**Acceptance Scenarios**:

1. **Given** the machine has no network access, **When** the user runs import → detect → review → export → verify, **Then** the core workflow completes without requiring connectivity.
2. **Given** a job completes or is cancelled, **When** temporary working files are cleaned up per product policy, **Then** sensitive intermediates are not left as an unlabeled long-term store.

---

### Edge Cases

- Original path equals chosen export path: export MUST be blocked so the original cannot be overwritten.
- Chosen export filename already exists in the output directory: that document’s export MUST be blocked with a conflict status/error; other batch documents continue.
- Empty detection results: user may still add manual regions; export with zero approved regions SHOULD warn and require confirmation or block empty “redaction” export [see Assumptions].
- Uncertain detection confidence: items MUST NOT be silently treated as confirmed matches or auto-approved.
- Partial OCR / partial page coverage: document MUST NOT be labeled as fully analyzed for all rules.
- Batch with mixed statuses: per-document outcomes remain independent; one failure MUST NOT mark others verified incorrectly.
- Password retained only as long as needed for the session/job; MUST NOT write passwords into exports or reports by default.
- Very large PDFs or large batches: UI remains responsive enough to cancel; failures surface per document.
- Forms, tables, and embedded images containing sensitive content: where permanent removal cannot be guaranteed for a selected region, export MUST fail safely for that guarantee path (failed/inconclusive), not claim verified secure redaction.
- Visual-only overlay path: MUST NOT be offered or labeled as secure/permanent redaction.

## Requirements *(mandatory)*

### Functional Requirements

#### Document management

- **FR-001**: Users MUST be able to import a single PDF into a workspace.
- **FR-002**: Users MUST be able to import multiple PDFs into the same workspace (batch).
- **FR-003**: The system MUST show per-document processing status (at least: imported, analyzing, ready for review, approved for export, exporting, verified, failed, inconclusive, unsupported).
- **FR-004**: The system MUST accept text PDFs, scanned PDFs, mixed PDFs, and encrypted PDFs (with user-supplied credentials), including documents that contain tables, forms, and embedded images, within declared capability limits.
- **FR-005**: The system MUST handle invalid, corrupted, and unsupported documents with explicit status and reason without crashing the workspace or blocking unrelated documents.

#### PDF viewer

- **FR-006**: Users MUST be able to view rendered pages and navigate between pages.
- **FR-007**: Users MUST be able to zoom and rotate the view for review.
- **FR-008**: Users MUST be able to select text where native text is available.
- **FR-009**: Users MUST be able to draw rectangular redaction regions on pages.
- **FR-010**: Users MUST be able to create multiple redaction regions across multiple pages.
- **FR-011**: The system MUST display proposed redactions before permanent application.

#### Detection engine

- **FR-012**: The system MUST extract text from native (text-based) PDFs locally.
- **FR-013**: The system MUST perform OCR locally on scanned content; OCR MUST NOT require network access.
- **FR-014**: The system MUST detect configurable sensitive information using enabled rules.
- **FR-015**: The system MUST ship built-in detection patterns for: PAN, Aadhaar, GSTIN, IFSC, bank accounts, phone numbers, email addresses, addresses, names, case references, and financial identifiers.
- **FR-016**: Users MUST be able to define custom regex, keywords, phrases, and validation rules.
- **FR-017**: Users MUST be able to enable and disable individual rules.
- **FR-018**: Users MUST be able to export and import rule configurations as JSON.
- **FR-019**: The system MUST display detection confidence and source information (e.g., native text vs OCR; rule identity).
- **FR-020**: The system MUST NOT silently treat uncertain detections as confirmed matches or as user approvals.

#### Review workspace

- **FR-021**: The system MUST display detections grouped by document and by category/rule.
- **FR-022**: Users MUST be able to navigate from a detection to its exact page and location.
- **FR-023**: Users MUST be able to select individual detections.
- **FR-024**: Users MUST be able to select all occurrences of a rule or category.
- **FR-025**: Users MUST be able to perform bulk selection across multiple documents in a batch.
- **FR-026**: Users MUST be able to reject false positives.
- **FR-027**: Users MUST be able to manually add redaction regions that enter the same approval flow.
- **FR-028**: The system MUST require explicit user review/approval before applying permanent redactions to an export.

#### Redaction and export

- **FR-029**: The system MUST support text-based and area-based redaction targets.
- **FR-030**: Permanent redaction MUST remove or irreversibly destroy underlying sensitive content (text, images, and vector content in scope for the approved regions)—not merely cover it visually.
- **FR-031**: Exports MUST be written as new output files; originals MUST remain unmodified.
- **FR-032**: Before presenting a completed export, the system MUST sanitize relevant metadata and hidden document information according to the product’s sanitization policy for that job.
- **FR-033**: The product MUST NEVER claim secure/permanent redaction for a visual overlay alone.
- **FR-034**: When secure redaction cannot be guaranteed for the approved set, the system MUST fail safely (failed or inconclusive)—not mark the export verified/successful.

#### Batch processing

- **FR-035**: Users MUST be able to apply a common detection rule set to multiple PDFs in one workspace.
- **FR-036**: Users MUST be able to review all batch results before permanent application.
- **FR-037**: The system MUST process only documents that meet approval requirements for export.
- **FR-038**: The system MUST show individual document status and errors during batch operations.
- **FR-039**: Users MUST be able to retry failed documents without redoing successful ones.
- **FR-040**: Users MUST choose an output directory for exports.
- **FR-041**: The system MUST prevent accidental overwriting of original documents (block same-path overwrite; refuse export targets that would replace sources).
- **FR-041a**: When a planned export filename already exists in the output directory, the system MUST block that document’s export, report a file-conflict error/status, and continue processing other batch documents (no auto-suffix, no silent overwrite, no per-file prompt required in MVP).

#### PDF utilities

- **FR-042**: Users MUST be able to reorder pages in the working/export preparation document without modifying the original source file.
- **FR-042a**: Users MUST be able to rotate pages in the working/export preparation document without modifying the original source file.
- **FR-042b**: Users MUST be able to delete pages in the working/export preparation document without modifying the original source file.
- **FR-042c**: MVP MUST NOT include highlight or annotation tools; such features, if added later, MUST remain distinct from permanent redaction and MUST NOT alone satisfy FR-030.

#### Verification and reporting

- **FR-043**: The system MUST verify that redacted text targeted by approved redactions is no longer extractable from the export by the product’s verification checks.
- **FR-044**: Verification MUST include relevant checks for image and vector content tied to approved regions, within declared capability.
- **FR-045**: Verification MUST inspect metadata and relevant document structures for residual sensitive material covered by the sanitization/verification policy.
- **FR-046**: The system MUST attempt to detect residual or recoverable sensitive content related to approved redactions and report findings.
- **FR-047**: The system MUST generate a verification report for every exported document.
- **FR-048**: Reports MUST distinguish verified, failed, and inconclusive outcomes with reasons.
- **FR-049**: Incomplete, skipped, or errored verification MUST NEVER be marked successful/verified.

#### Security and privacy

- **FR-050**: Core import, OCR, detection, review, redaction, sanitization, verification, and export MUST run with local processing only.
- **FR-051**: The system MUST NOT send document contents or page images to external services for core functionality.
- **FR-052**: The system MUST NOT include document telemetry (no content, no file paths in analytics events).
- **FR-053**: Core functionality MUST NOT depend on external network services.
- **FR-054**: Temporary files and passwords MUST be handled securely (minimal lifetime; not written into exports/reports by default).
- **FR-055**: The system MUST NOT silently retain sensitive document contents in hidden long-term stores after job completion or cancel, beyond user-visible outputs they chose to save.
- **FR-055a**: Rule configurations (built-in enablement and custom rules) MUST persist locally across application restarts.
- **FR-055b**: Document workspace state (imported documents, detections, approvals, and in-progress review) MUST be session-scoped for MVP and MUST NOT be required to survive application restart; durable user outputs are exports and verification reports the user saved, plus persisted rule configurations.
- **FR-056**: Encrypted and malformed PDFs MUST surface clear, actionable errors.

#### Usability and accessibility

- **FR-057**: Review and approval flows MUST be operable via keyboard for primary actions (navigate detections, approve/reject, confirm export).
- **FR-058**: Status and errors MUST NOT rely on color alone (text/labels required).
- **FR-059**: Focus order in the review workspace MUST be logical for assistive technologies in the desktop UI host.
- **FR-060**: Destructive or permanent actions (apply redaction/export) MUST require explicit confirmation when the approved set will permanently remove content in the export.

#### Out of scope (non-goals)

- **FR-061**: The MVP MUST NOT require user authentication.
- **FR-062**: The MVP MUST NOT provide cloud processing of documents.
- **FR-063**: The MVP MUST NOT provide team collaboration features.
- **FR-064**: The MVP MUST NOT provide digital signature features.
- **FR-065**: The MVP MUST NOT include subscription or payment systems.

### Key Entities

- **Workspace**: An ephemeral local session containing one or more documents under review, shared rule configuration selection, and batch actions; not durable across restarts in MVP.
- **Document**: A user-imported PDF reference (source path), processing status, unlock state, analysis artifacts, and export history references within the current session.
- **Persisted Rule Configuration**: Locally stored rule enablement and custom rules that survive application restarts (distinct from ephemeral workspace documents).
- **Detection**: A proposed sensitive match with rule/category, location (page + region), confidence, source (native/OCR), and review state (proposed/approved/rejected).
- **Manual Region**: User-drawn area intended for redaction; follows the same approval rules as detections.
- **Rule**: Built-in or custom detection definition; enablement flag; optional validation; portable via JSON rule configuration.
- **Rule Configuration**: Named set of built-in toggles plus custom rules; import/exportable as JSON.
- **Approval Set**: The user-approved detections and manual regions eligible for a given export.
- **Export Artifact**: New output PDF path chosen by the user; never the original source path.
- **Verification Report**: Per-export record with outcome (verified | failed | inconclusive), checks performed, failures/residuals, and timestamps.
- **Document Lifecycle State**: imported → (unlock if needed) → analyzing → ready_for_review → partially_reviewed → approved_for_export → exporting → verified | failed | inconclusive | unsupported.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A professional can complete a single-document secure path (import → review → export with verification outcome) for a typical under-20-page text PDF in under 10 minutes without network access.
- **SC-002**: In 100% of successful exports marked **verified**, approved targeted text samples used in acceptance tests are not extractable from the export via the product’s verification extraction checks.
- **SC-003**: In 100% of cases where verification is incomplete or fails, the UI and report do **not** show verified/successful redaction status.
- **SC-004**: In 100% of export jobs, the original source file byte contents remain unchanged after the job.
- **SC-005**: 100% of core workflow steps (import, detect, review, export, verify) succeed with network disabled in acceptance tests.
- **SC-006**: For a batch of at least 5 PDFs, users can see per-document status and can export only approved documents while leaving unapproved documents unexported.
- **SC-007**: At least 90% of first-time professional testers (CA/legal) can approve/reject a detection and locate it on the page without assistance in moderated usability tests.
- **SC-008**: Keyboard-only users can complete approve/reject and export confirmation for a single-document flow in acceptance accessibility tests.
- **SC-009**: Corrupted and wrong-password PDFs produce a visible failed/unsupported state within 30 seconds of the failed attempt under normal local hardware assumptions, without terminating the whole workspace.
- **SC-010**: Rule JSON round-trip (export then import) restores enablement and custom rules with no manual re-entry for the tested configuration.
- **SC-011**: After application restart, previously saved rule configuration remains available without re-import; previously open documents/detections/approvals are not required to reappear.
- **SC-012**: In a batch export where at least one target filename already exists in the output folder, that document is reported as a file conflict (not verified), is not overwritten, and other non-conflicting documents can still complete independently.

## Assumptions

- Secure MVP prioritizes: document import, local detection (built-in patterns + toggles), review/approval, permanent redaction export, sanitization, verification reports, batch basics, offline operation, and original preservation.
- Custom rule authoring and JSON import/export are in product scope; if delivery is phased, built-in toggles ship with the first secure export path and custom/JSON follow immediately after without changing security gates.
- “Names” and “addresses” built-in detection are best-effort pattern/heuristic detectors; uncertain matches remain visible as uncertain and never auto-approved (FR-020).
- Empty approval set: exporting with zero approved regions is blocked or requires an extra confirmation that no redactions will be applied (product should prefer block for MVP clarity).
- Platforms in scope are Windows and macOS desktop; mobile/web SaaS are out of scope.
- Single local user per machine install; no multi-account profiles required for MVP.
- Verification cannot claim absolute resistance to all forensic lab techniques; outcomes are limited to the product’s documented checks—copy MUST not overclaim (constitution-aligned).
- Page reorder/rotate/delete operate on working/export preparation state only; originals remain immutable (FR-031, FR-041).
- Highlights/annotations are out of MVP; if added later they never satisfy FR-030 alone.

## MVP vs Future Enhancements

### Secure MVP (must ship together)

- Import single/multiple PDFs; statuses; graceful failure
- Viewer navigation, zoom, view rotate, text select, rectangle regions, proposal preview
- Page utilities: reorder, rotate, delete (working/export path only; originals untouched)
- Local text extraction + local OCR; built-in rule set listed in FR-015 with enable/disable
- Review workspace with grouping, navigation, select/bulk select, reject, manual regions, mandatory approval
- Permanent redaction export to new files; sanitization; fail-safe behavior; no overlay-as-secure
- Batch apply rules, review, approved-only process, per-doc errors, retry, output directory, no original overwrite
- Export name collisions: block conflicting file, report error, continue other documents
- Persisted rule configurations; ephemeral document/session workspace
- Verification + report with verified/failed/inconclusive; never mark incomplete as success
- Offline, no document telemetry, secure temp/password handling

### Future enhancements

- Highlights and annotations (kept distinct from permanent redaction if introduced)
- Durable multi-document workspace resume across restarts
- Auto-suffix or interactive rename policies for export collisions
- Team libraries of shared cloud rule packs (local JSON sharing remains; cloud sync out of scope)
- Digital signatures, authentication, collaboration, payments (out of scope)
- Richer forensic-grade verification beyond documented checks
- Automated redaction without review (explicitly rejected by product principles)

## Clarifications

### Resolved

- **Q1 (PDF utilities)**: Option B — Include page reorder, rotate, and delete in MVP; defer highlights/annotations (FR-042–FR-042c).
- **Q2 (Persistence)**: Option C — Persist rule configurations only; document/session workspace is ephemeral across restarts (FR-055a, FR-055b, SC-011).
- **Q3 (Export collisions)**: Option B — Block conflicting files, report conflict, continue other documents (FR-041a, SC-012).
