<!--
Sync Impact Report
- Version change: (none) → 1.0.0
- Modified principles: N/A (initial ratification; replaced template placeholders)
- Added sections:
  - Core Principles I–XII (full Redact engineering principles)
  - Non-Negotiable Product & Platform Constraints
  - Quality Gates & Verification
  - Governance
- Removed sections: template placeholder sections only
- Follow-up TODOs: none deferred; ratification date set to first adoption (2026-10-30)
-->

# Redact Constitution

## Core Principles

### I. Privacy and Security by Design (NON-NEGOTIABLE)
Redact MUST treat user documents and derived artifacts as confidential by default.
Security requirements take precedence over convenience, performance shortcuts, and UX
polish when those conflict.

- MUST NOT require authentication, accounts, backends, mandatory cloud services, or
  document uploads to operate core redaction workflows.
- MUST keep document bytes, OCR output, detection results, and export artifacts on the
  local machine unless the user explicitly opts into a clearly labeled, optional local
  integration they control.
- MUST NOT invent, imply, or advertise security guarantees the product cannot
  demonstrate with tests and verification evidence (for example, "military-grade,"
  "unrecoverable against all forensic tools," or "certified compliant" without an
  actual certification scope).
- MUST minimize retention of intermediate artifacts; temporary working files MUST be
  scoped to a job and removable after successful completion or explicit user cancel.

**Rationale**: A redaction tool that exfiltrates or over-claims trust is worse than no
tool. Measurable privacy is locality of data plus honest threat-model boundaries.

### II. Secure PDF Processing Requirements (NON-NEGOTIABLE)
Redaction MUST be genuine permanent content removal or irreversible content destruction
in the exported PDF—not visual overlays, white boxes, annotations, or UI-only hides.

- MUST remove or irreversibly destroy the targeted content (text operators, image
  samples, form field values, and other selected objects) such that the content is not
  recoverable via copy/paste, text extraction, or reopening the export in a standards
  PDF viewer.
- MUST run export-path sanitization before any file is presented as a completed export
  (examples of required classes of cleanup: strip or neutralize remaining sensitive
  metadata where configured, remove unused objects related to redacted content as
  implemented by the engine, and avoid leaving recoverable soft-mask-only hides).
- MUST run verification after sanitization. An export MUST NOT be labeled, surfaced, or
  logged as **verified** when verification is incomplete, skipped, errored, or failed.
- MUST fail closed: if sanitization or verification cannot complete successfully, the
  product MUST withhold verified status, surface the failure, and MUST NOT silently
  substitute an unverified file as verified.
- MUST support processing of text PDFs, scanned PDFs, mixed PDFs, encrypted PDFs (with
  user-supplied credentials), and documents containing tables, forms, and embedded
  images—within documented engine capability limits. Unsupported cases MUST be
  reported explicitly rather than silently under-redacted.

**Rationale**: Overlay "redaction" creates false confidence. Verification is a gate,
not a badge of convenience.

### III. Original-Document Immutability (NON-NEGOTIABLE)
Original source documents MUST never be modified in place.

- MUST treat user-selected source files as read-only inputs.
- MUST write all redacted output to new paths chosen or confirmed by the user (or to
  explicitly staged export locations), never by overwriting the source path.
- MUST NOT mutate originals for OCR caches, decryption working copies, or bulk jobs;
  any derived working copy MUST be distinct from the source file identity.
- MUST make source-vs-export path separation visible in the UI for single and bulk jobs.

**Rationale**: Irreversible mistakes on originals are unacceptable for a security tool.

### IV. Modular Architecture and Engine Abstraction
PDF engines, OCR engines, detectors, sanitizers, and verifiers MUST be modular and
behind stable abstractions.

- MUST isolate engine-specific code behind interfaces/contracts so engines can be
  replaced or upgraded without rewriting UI or domain orchestration.
- MUST keep detection rules configurable and extensible without forking core UI.
- MUST version engine capabilities (supported document classes, encryption modes,
  verification checks) so the UI can gate features honestly.
- SHOULD prefer small, composable pipeline stages (ingest → analyze → detect → review →
  redact → sanitize → verify → export) with explicit stage inputs/outputs.

**Rationale**: PDF stacks evolve; abstraction prevents lock-in and enables tested swaps.

### V. Strict Separation of UI, Domain Logic, and Infrastructure
The codebase MUST enforce clear boundaries among presentation, domain, and
infrastructure.

- **UI** (React/TypeScript in the Tauri webview): rendering, interaction, accessibility,
  and presentation state only. MUST NOT embed redaction algorithms, PDF mutation, or
  verification logic.
- **Domain**: redaction job models, review/approval state machines, rule evaluation
  policies, and export eligibility rules. MUST be testable without a GUI.
- **Infrastructure** (Rust/Tauri commands, filesystem, local OCR/PDF libraries): I/O,
  engine adapters, and process isolation. MUST NOT own product policy beyond executing
  domain decisions.
- Cross-boundary calls MUST use explicit contracts (commands/events/DTOs). UI MAY NOT
  reach into engine crates except through approved command APIs.

**Rationale**: Boundary discipline keeps security-critical logic reviewable and tested.

### VI. Offline-First Engineering (NON-NEGOTIABLE)
Redact MUST be a local-first, offline-capable, single-user desktop application.

- Core ingest, OCR, detection, review, redaction, sanitization, verification, and export
  MUST work with no network connectivity.
- MUST NOT hard-depend on remote APIs for OCR or detection.
- Optional network features (if any) MUST be off by default, clearly labeled, and
  disableable without breaking core workflows.
- Telemetry, if ever introduced, MUST be opt-in, documented, and free of document
  contents and file paths by default policy.

**Rationale**: Offline operation is both a privacy control and a product requirement.

### VII. Test-Driven Development (NON-NEGOTIABLE)
Security-sensitive and domain behavior MUST be developed test-first.

- For domain rules, approval gates, export eligibility, sanitization outcomes, and
  verification status transitions: write failing tests → implement → refactor.
- Engine adapter changes that affect redaction permanence or verification MUST include
  automated fixtures (corpus PDFs) asserting extractable content is gone for approved
  redaction regions and that failed verification cannot yield verified status.
- UI behavior around review/approval MUST be covered by component or end-to-end tests
  for the non-bypassability of approval before export.

**Rationale**: Redaction defects are high-impact; tests are the primary safety net.

### VIII. Automated Security and Regression Testing (NON-NEGOTIABLE)
Every merge MUST be protected by automated suites that catch security and regression
failures.

- MUST maintain regression corpora for text, scanned, mixed, form/table, image-bearing,
  and encrypted PDFs (with test credentials stored as test secrets, not production
  secrets).
- MUST automate checks that unverified or failed verification paths cannot be marked
  verified.
- MUST automate checks that source files remain byte-identical after jobs (immutability).
- MUST run dependency vulnerability scanning in CI for Rust and JavaScript ecosystems.
- Security fixes MUST include a regression test that fails before the fix lands.

**Rationale**: Manual spot-checks do not scale to bulk processing or engine upgrades.

### IX. Accessibility and Usability
Review and approval workflows MUST be usable and accessible; security MUST NOT depend
on obscure UI alone.

- MUST support keyboard-operable review/approval of detected regions before export.
- MUST meet baseline accessibility expectations for desktop webview UI (semantic
  structure, focus order, contrast for status/error states, non-color-only status).
- MUST present clear states: detected, user-approved, rejected, pending verification,
  verified, failed.
- Bulk operations MUST show per-document progress and per-document failure reasons
  without requiring silent skip of failures.
- Error messages MUST distinguish user-correctable issues (password needed, unsupported
  feature) from verification/sanitization failures.

**Rationale**: Users must understand what they approve; inaccessible review invites
bypass or rubber-stamping.

### X. Open-Source Licensing and Dependency Governance
Redact is open source. Dependencies and licenses MUST be deliberate and reviewable.

- MUST maintain an inventory of third-party dependencies (Rust + JS) and their licenses.
- MUST NOT add dependencies that conflict with the project’s chosen open-source license
  without an explicit governance decision recorded in the repository.
- MUST prefer maintained libraries with clear security track records for PDF/OCR paths.
- MUST document outbound network behavior of any dependency used at runtime.
- Pull requests that add or upgrade security-critical native dependencies MUST note
  risk and test impact.

**Rationale**: Supply-chain and license debt become user risk in a security product.

### XI. Cross-Platform Compatibility
Redact MUST support Windows and macOS as first-class platforms on Tauri 2.

- Feature work MUST be validated on both Windows and macOS before release claims.
- Platform-specific code MUST be isolated behind infrastructure adapters; domain logic
  MUST remain platform-agnostic.
- Filesystem path handling, fonts, and PDF/OCR native libs MUST have CI or documented
  manual release checks on both targets.
- MUST NOT ship platform-only critical security behavior without an equivalent control
  on the other supported OS.

**Rationale**: Unequal platforms create uneven privacy outcomes for users.

### XII. Performance and Resource Management
Local OCR, detection, and bulk processing MUST bound resource use and remain
responsive.

- MUST process documents in bounded jobs with cancellable operations.
- MUST avoid unbounded memory growth on large or bulk PDF sets (streaming/chunking or
  explicit pagination of work where required).
- MUST keep the UI responsive during OCR/detection; heavy work belongs off the UI
  thread / in Rust side tasks.
- MUST expose progress for long-running OCR, detection, sanitization, and verification.
- Performance optimizations MUST NOT weaken immutability, approval gates, sanitization,
  or verification requirements.

**Rationale**: Bulk local processing is a feature only if it remains safe and usable.

## Non-Negotiable Product & Platform Constraints

- **Stack**: Tauri 2, React, TypeScript, and Rust. New major runtime/UI frameworks
  require a constitution amendment.
- **Distribution model**: single-user desktop app; not a multi-tenant SaaS.
- **Human approval**: all redactions that will appear in an export require explicit user
  review and approval; auto-detect MUST NOT auto-approve for export.
- **Detection**: rules are configurable and extensible; OCR and detection MUST run
  locally.
- **Bulk**: multi-document jobs are supported; per-document success/failure MUST be
  tracked independently.
- **Precedence**: when usability, speed, or feature ambition conflicts with Principles
  I–III, VI, VII, or VIII, those principles win.

## Quality Gates & Verification

A change is not release-ready unless:

1. Automated tests cover affected domain gates (approval → export eligibility).
2. Sanitization + verification behavior is tested for success and failure paths,
   including the rule that incomplete/failed verification cannot yield verified status.
3. Source immutability checks pass for touched job flows.
4. No new mandatory network dependency is introduced for core workflows.
5. Security-relevant user copy avoids unverifiable absolute guarantees.
6. Windows and macOS impact is assessed for native/engine changes.

**Measurable definitions**:

- **Verified export**: sanitization completed successfully AND verification completed
  successfully AND all verification checks configured for that job passed.
- **Approved redaction set**: user-explicit acceptances recorded for regions/items
  included in the export; unapproved detections MUST NOT be applied as redactions in
  that export.
- **Source integrity**: cryptographic hash (or equivalent byte compare) of the original
  file is unchanged after the job.

## Governance

This constitution supersedes informal practice, convenience-driven shortcuts, and
unreviewed "temporary" security exceptions. All specs, plans, and PRs for Redact MUST
be consistent with it.

- **Amendments**: require a PR that updates this file, adjusts `CONSTITUTION_VERSION`
  using SemVer (MAJOR: remove/redefine non-negotiables; MINOR: add/expand principle;
  PATCH: clarify wording), sets **Last Amended** to the adoption date, and includes a
  Sync Impact Report comment for reviewers (removed before or at merge as project
  practice dictates).
- **Conflicts**: resolve in favor of stronger privacy/security and fail-closed export
  behavior.
- **Compliance review**: reviewers MUST check Principles I–III and verification labeling
  on any PR touching PDF pipelines, export UX, or detection approval flows.
- **Exceptions**: time-boxed exceptions require written justification in the PR,
  linked tests or follow-up issues, and an expiration condition. Silent exceptions are
  invalid.
- **Guidance**: feature-level requirements live in Spec Kit specs/plans; this document
  defines enduring engineering law only.

**Version**: 1.0.0 | **Ratified**: 2026-10-30 | **Last Amended**: 2026-10-30
