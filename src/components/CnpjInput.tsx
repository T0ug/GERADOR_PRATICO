import { CnpjValidationState, sanitizeCnpj } from "../utils/cnpj";

type CnpjInputProps = {
  value: string;
  validationState: CnpjValidationState;
  onChange: (value: string) => void;
};

const validationMessages: Record<CnpjValidationState, string> = {
  empty: "Informe o CNPJ da empresa para continuar.",
  valid: "CNPJ valido.",
  invalid: "CNPJ invalido. Confira os numeros informados.",
};

export function CnpjInput({ value, validationState, onChange }: CnpjInputProps) {
  const sanitized = sanitizeCnpj(value);
  const statusClass = `validation-message ${validationState}`;

  return (
    <section className="panel" aria-labelledby="cnpj-title">
      <div className="panel-heading">
        <div>
          <p className="section-label">Empresa</p>
          <h2 id="cnpj-title">CNPJ da empresa</h2>
        </div>
        <span className="pill">Obrigatorio</span>
      </div>

      <label className="field">
        <span>CNPJ</span>
        <input
          value={value}
          inputMode="numeric"
          placeholder="00.000.000/0000-00"
          aria-invalid={validationState === "invalid"}
          aria-describedby="cnpj-validation-message"
          onChange={(event) => onChange(event.target.value)}
        />
      </label>
      <p id="cnpj-validation-message" className={statusClass}>
        {validationMessages[validationState]}
      </p>
      <p className="field-note">
        Numeros identificados: {sanitized.length}/14. O CNPJ pode ser informado com ou sem mascara.
      </p>
    </section>
  );
}
