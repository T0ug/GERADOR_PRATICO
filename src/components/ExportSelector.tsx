type ExportSelectorProps = {
  value: string;
  onChange: (value: string) => void;
  onSelectClick: () => void;
};

export function ExportSelector({ value, onChange, onSelectClick }: ExportSelectorProps) {
  return (
    <section className="panel" aria-labelledby="export-title">
      <div className="panel-heading">
        <div>
          <p className="section-label">Saida</p>
          <h2 id="export-title">Arquivo Excel</h2>
        </div>
      </div>

      <label className="field">
        <span>Local de exportacao</span>
        <input
          value={value}
          placeholder="Relatorio de notas Jan-2026.xlsx"
          onChange={(event) => onChange(event.target.value)}
        />
      </label>
      <button type="button" className="secondary-action full-width" onClick={onSelectClick}>
        Escolher local
      </button>
      {value ? <p className="field-note selected-path">{value}</p> : null}
    </section>
  );
}
