type Props = { warning?: string | null };

export function CapabilityWarning({ warning }: Props) {
  if (!warning) return null;
  return (
    <div className="capability-banner" role="status">
      Capability limit: {warning}. Export will not be labeled verified.
    </div>
  );
}
