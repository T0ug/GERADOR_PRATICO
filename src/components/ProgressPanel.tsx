type ProgressPanelProps = {
  xmlCount: number;
  zipCount: number;
  stageLabel: string;
  message: string;
  percent: number;
  current: number;
  progressTotal: number;
};

export function ProgressPanel({
  xmlCount,
  zipCount,
  stageLabel,
  message,
  percent,
  current,
  progressTotal,
}: ProgressPanelProps) {
  const sourceCount = xmlCount + zipCount;
  const clampedPercent = Math.max(0, Math.min(100, percent));
  const detail = progressTotal > 0 ? `${current}/${progressTotal}` : "Aguardando inicio";

  return (
    <section className="panel" aria-labelledby="progress-title">
      <div className="panel-heading">
        <div>
          <p className="section-label">Processamento</p>
          <h2 id="progress-title">Progresso</h2>
        </div>
        <span className="pill">{sourceCount} entradas</span>
      </div>

      <div className="progress-track" aria-label="Progresso visual">
        <div className="progress-fill" style={{ width: `${clampedPercent}%` }} />
      </div>
      <p className="field-note">{stageLabel}</p>
      <p className="field-note">{message}</p>
      <p className="field-note">
        {clampedPercent}% concluido
        {progressTotal > 0 ? ` - ${detail}` : ""}
      </p>
    </section>
  );
}
