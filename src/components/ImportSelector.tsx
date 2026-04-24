type ImportSelectorProps = {
  selectedPaths: string[];
  onSelectFiles: () => void;
  onClear: () => void;
};

export function ImportSelector({
  selectedPaths,
  onSelectFiles,
  onClear,
}: ImportSelectorProps) {
  const previewItems = selectedPaths.slice(0, 4);
  const hiddenCount = Math.max(selectedPaths.length - previewItems.length, 0);

  return (
    <section className="panel" aria-labelledby="import-title">
      <div className="panel-heading">
        <div>
          <p className="section-label">Importacao</p>
          <h2 id="import-title">Arquivos fiscais</h2>
        </div>
      </div>

      <div className="button-grid">
        <button type="button" className="secondary-action" onClick={onSelectFiles}>
          Selecionar arquivos
        </button>
        <button type="button" className="ghost-action" onClick={onClear}>
          Limpar selecao
        </button>
      </div>

      <div className="file-summary" aria-live="polite">
        <span>{selectedPaths.length} arquivo(s) selecionado(s)</span>
      </div>

      {previewItems.length > 0 ? (
        <ul className="path-preview" aria-label="Arquivos selecionados">
          {previewItems.map((path) => (
            <li key={path}>{path}</li>
          ))}
          {hiddenCount > 0 ? <li>Mais {hiddenCount} arquivo(s) selecionado(s)</li> : null}
        </ul>
      ) : null}
    </section>
  );
}
