import { useState } from "react";

export type DescriptionMode = "complete" | "limited";

type DescriptionOptionsProps = {
  mode: DescriptionMode;
  wordLimit: number;
  onModeChange: (mode: DescriptionMode) => void;
  onWordLimitChange: (value: number) => void;
};

export function DescriptionOptions({
  mode,
  wordLimit,
  onModeChange,
  onWordLimitChange,
}: DescriptionOptionsProps) {
  const isLimited = mode === "limited";
  const limitedExample = Math.max(1, Math.min(wordLimit, 3));
  const [isHelpOpen, setIsHelpOpen] = useState(false);

  return (
    <section className="panel" aria-labelledby="description-title">
      <div className="panel-heading">
        <div className="panel-heading-main">
          <p className="section-label">Descricao dos itens</p>
          <h2 id="description-title">Formato no Excel</h2>
        </div>
        <button
          type="button"
          className="help-button"
          aria-label="Ver exemplo de descricao completa e limitada"
          aria-expanded={isHelpOpen}
          onClick={() => setIsHelpOpen((current) => !current)}
        >
          ?
        </button>
      </div>

      <div className="segmented-control" role="radiogroup" aria-label="Formato da descricao">
        <label>
          <input
            type="radio"
            name="description-mode"
            checked={mode === "complete"}
            onChange={() => onModeChange("complete")}
          />
          <span>Completa</span>
        </label>
        <label>
          <input
            type="radio"
            name="description-mode"
            checked={isLimited}
            onChange={() => onModeChange("limited")}
          />
          <span>Limitada</span>
        </label>
      </div>

      {isHelpOpen ? (
        <div className="description-help">
          <p className="field-note">
            Completa: FILTRO DE AR PECAS SCANIA; KIT WLM PARTICIPACOES E COMERCIO
          </p>
          <p className="field-note">
            Limitada ({limitedExample} palavras): FILTRO DE AR; KIT WLM PARTICIPACOES
          </p>
        </div>
      ) : null}

      {isLimited ? (
        <label className="field compact-field">
          <span>Limite de palavras por item</span>
          <input
            type="number"
            min="1"
            max="100"
            value={wordLimit}
            onChange={(event) => onWordLimitChange(Number(event.target.value))}
          />
        </label>
      ) : null}
    </section>
  );
}
