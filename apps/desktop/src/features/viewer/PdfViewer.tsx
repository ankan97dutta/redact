/** PDF.js render-only viewer shell — no PDF mutation. */

type Props = {
  sourcePath: string | null;
  highlightPage?: number;
};

export function PdfViewer({ sourcePath, highlightPage }: Props) {
  return (
    <section aria-label="PDF viewer" className="viewer">
      <p className="viewer-note">
        Render-only viewer (PDF.js). Permanent redaction runs in Rust engines only.
      </p>
      {sourcePath ? (
        <p>
          Loaded: {sourcePath}
          {highlightPage != null ? ` · focus page ${highlightPage + 1}` : null}
        </p>
      ) : (
        <p>Select a document to preview.</p>
      )}
    </section>
  );
}
