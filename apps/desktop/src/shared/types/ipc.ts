/** IPC DTOs — verification outcome is read-only; UI cannot set verified. */

export type DocumentStatus =
  | "imported"
  | "needs_password"
  | "analyzing"
  | "ready_for_review"
  | "reviewing"
  | "approved_for_export"
  | "exporting"
  | "verified"
  | "failed"
  | "inconclusive"
  | "unsupported"
  | "cancelled";

export type ReviewState = "proposed" | "approved" | "rejected";

/** Outcome is only returned from the backend — never accepted as command input. */
export type VerificationOutcome = "verified" | "failed" | "inconclusive";

export type DocumentSummary = {
  id: string;
  source_path: string;
  display_name: string;
  status: DocumentStatus;
  capability_warning?: string | null;
  last_error?: string | null;
};

export type CanonicalRectDto = {
  page_index: number;
  x0: number;
  y0: number;
  x1: number;
  y1: number;
};

export type DetectionDto = {
  id: string;
  document_id: string;
  rule_id: string;
  category: string;
  review_state: ReviewState;
  confidence: string;
  source: string;
  rect: CanonicalRectDto;
  snippet: string;
  page_index: number;
};

export type VerificationReportDto = {
  outcome: VerificationOutcome;
  checks: { id: string; completed: boolean; passed: boolean; detail: string }[];
  reasons: string[];
};
