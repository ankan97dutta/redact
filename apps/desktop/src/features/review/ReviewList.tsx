import { invoke } from "@tauri-apps/api/core";
import type { DetectionDto, ReviewState } from "../../shared/types/ipc";

type Props = {
  detections: DetectionDto[];
  onChanged: () => void;
  onNavigate: (pageIndex: number) => void;
};

export function ReviewList({ detections, onChanged, onNavigate }: Props) {
  async function setState(ids: string[], state: ReviewState) {
    await invoke("set_detection_review_state", { input: { ids, state } });
    onChanged();
  }

  return (
    <section aria-label="Review detections">
      <h2>Detections</h2>
      <ul>
        {detections.map((d) => (
          <li key={d.id}>
            <button type="button" onClick={() => onNavigate(d.page_index)}>
              {d.rule_id}: {d.snippet}
            </button>
            <span>{d.review_state}</span>
            <button type="button" onClick={() => void setState([d.id], "approved")}>
              Approve
            </button>
            <button type="button" onClick={() => void setState([d.id], "rejected")}>
              Reject
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
