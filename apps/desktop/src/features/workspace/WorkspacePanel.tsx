import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import type { DocumentSummary } from "../../shared/types/ipc";

type Props = {
  documents: DocumentSummary[];
  selectedId: string | null;
  onImported: (docs: DocumentSummary[]) => void;
  onSelect: (id: string) => void;
  onAnalyze: (id: string) => void;
};

export function WorkspacePanel({
  documents,
  selectedId,
  onImported,
  onSelect,
  onAnalyze,
}: Props) {
  async function importPdfs() {
    const selected = await open({
      multiple: true,
      filters: [{ name: "PDF", extensions: ["pdf"] }],
    });
    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    const result = await invoke<{ documents: DocumentSummary[] }>("import_documents", {
      input: { paths },
    });
    onImported(result.documents);
  }

  return (
    <section aria-label="Workspace">
      <header className="panel-header">
        <h2>Documents</h2>
        <button type="button" onClick={() => void importPdfs()}>
          Import PDFs
        </button>
      </header>
      <ul className="doc-list">
        {documents.map((d) => (
          <li key={d.id}>
            <button
              type="button"
              className={selectedId === d.id ? "doc selected" : "doc"}
              onClick={() => onSelect(d.id)}
            >
              <span>{d.display_name}</span>
              <span className="status" aria-label={`Status ${d.status}`}>
                {d.status}
              </span>
            </button>
            {d.capability_warning ? (
              <p className="warn" role="status">
                {d.capability_warning}
              </p>
            ) : null}
            <button type="button" onClick={() => void onAnalyze(d.id)}>
              Analyze
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
