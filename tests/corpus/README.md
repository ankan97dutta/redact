# Synthetic corpus

All fixtures in this tree MUST be **synthetic**. Do not commit real client documents, real PII, or production scans.

## Policy

- Prefer generated PDFs from `redact-test-support`
- Use clearly fake identifiers (e.g. `PAN AAAPA1234A` patterns reserved for tests)
- Document fixture purpose in the filename or a sibling `.md` note

## Planned fixtures

| Fixture | Story | Purpose |
|---------|-------|---------|
| `text_pan_sample.pdf` | US1 | Native text secure path |
| `scan_gstin_sample.pdf` | US3 | OCR detection |
| encrypted / corrupt samples | US4 | Unlock and failure paths |
