type GtinsOptionsProps = {
  extractGtins: boolean;
  splitGtinsByOperation: boolean;
  onExtractGtinsChange: (value: boolean) => void;
  onSplitGtinsByOperationChange: (value: boolean) => void;
};

export function GtinsOptions({
  extractGtins,
  splitGtinsByOperation,
  onExtractGtinsChange,
  onSplitGtinsByOperationChange,
}: GtinsOptionsProps) {
  return (
    <section className="panel compact-panel" aria-labelledby="gtins-title">
      <div className="panel-heading tight-heading">
        <div className="panel-heading-main">
          <p className="section-label">GTINS</p>
          <h2 id="gtins-title">Produtos no Excel</h2>
        </div>
      </div>

      <label className="toggle-row">
        <input
          type="checkbox"
          checked={extractGtins}
          onChange={(event) => onExtractGtinsChange(event.target.checked)}
        />
        <span className="toggle-control" aria-hidden="true" />
        <span>Extrair GTINS tambem?</span>
      </label>

      {extractGtins ? (
        <label className="toggle-row nested-toggle">
          <input
            type="checkbox"
            checked={splitGtinsByOperation}
            onChange={(event) => onSplitGtinsByOperationChange(event.target.checked)}
          />
          <span className="toggle-control" aria-hidden="true" />
          <span>Separar GTINS de entrada e saida em abas diferentes?</span>
        </label>
      ) : null}
    </section>
  );
}
