import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { WorkspacePanel } from "./features/workspace/WorkspacePanel";
import { CapabilityWarning } from "./features/workspace/CapabilityWarning";
import { PdfViewer } from "./features/viewer/PdfViewer";
import { ReviewList } from "./features/review/ReviewList";
import { ExportDialog } from "./features/export/ExportDialog";
import type { DetectionDto, DocumentSummary } from "./shared/types/ipc";
import "./styles.css";

type CapabilitiesSummary = {
  schema_version: string;
  app_version: string;
  limitations: string[];
};

export function App() {
  const [caps, setCaps] = useState<CapabilitiesSummary | null>(null);
  const [documents, setDocuments] = useState<DocumentSummary[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [detections, setDetections] = useState<DetectionDto[]>([]);
  const [pageFocus, setPageFocus] = useState<number | undefined>();

  useEffect(() => {
    invoke<CapabilitiesSummary>("get_capabilities").then(setCaps).catch(console.error);
  }, []);

  const selected = documents.find((d) => d.id === selectedId) ?? null;

  const refreshDetections = useCallback(async () => {
    if (!selectedId) return;
    const out = await invoke<{ detections: DetectionDto[] }>("get_detections", {
      documentId: selectedId,
    });
    setDetections(out.detections);
  }, [selectedId]);

  async function analyze(id: string) {
    const summary = await invoke<DocumentSummary>("analyze_document", { documentId: id });
    setDocuments((prev) => prev.map((d) => (d.id === id ? summary : d)));
    setSelectedId(id);
    await refreshDetections();
  }

  return (
    <main className="shell layout">
      <header className="brand">
        <h1>Redact</h1>
        <p className="tagline">Local-first PDF redaction for professionals</p>
        {caps ? (
          <p className="caps-line">
            v{caps.app_version} · {caps.limitations[0]}
          </p>
        ) : null}
      </header>

      <CapabilityWarning warning={selected?.capability_warning} />

      <div className="grid">
        <WorkspacePanel
          documents={documents}
          selectedId={selectedId}
          onImported={(docs) => {
            setDocuments((prev) => [...prev, ...docs]);
            if (docs[0]) setSelectedId(docs[0].id);
          }}
          onSelect={setSelectedId}
          onAnalyze={(id) => void analyze(id)}
        />
        <PdfViewer sourcePath={selected?.source_path ?? null} highlightPage={pageFocus} />
        <ReviewList
          detections={detections}
          onChanged={() => void refreshDetections()}
          onNavigate={setPageFocus}
        />
        <ExportDialog
          documentId={selectedId}
          onDone={(results) => {
            setDocuments((prev) =>
              prev.map((d) => results.find((r) => r.id === d.id) ?? d),
            );
          }}
        />
      </div>
    </main>
  );
}
