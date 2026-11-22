import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import type { DocumentSummary, VerificationOutcome } from "../../shared/types/ipc";

type Props = {
  documentId: string | null;
  onDone: (docs: DocumentSummary[]) => void;
};

export function ExportDialog({ documentId, onDone }: Props) {
  async function exportDoc() {
    if (!documentId) return;
    const confirmed = window.confirm(
      "Export will permanently remove approved content in a new file. Continue?",
    );
    if (!confirmed) return;

    await invoke("mark_document_approved_for_export", { documentId });
    const output = await open({ directory: true });
    if (!output || Array.isArray(output)) return;

    const result = await invoke<{ results: DocumentSummary[] }>("export_documents", {
      input: { document_ids: [documentId], output_dir: output },
    });
    onDone(result.results);
  }

  return (
    <section aria-label="Export">
      <button type="button" disabled={!documentId} onClick={() => void exportDoc()}>
        Approve & Export
      </button>
      <p className="hint">
        Empty approval sets are blocked. Outcomes: verified / failed / inconclusive.
      </p>
    </section>
  );
}

export function outcomeLabel(outcome: VerificationOutcome): string {
  return outcome;
}
