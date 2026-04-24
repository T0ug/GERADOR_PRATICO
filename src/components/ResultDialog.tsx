type ResultDialogProps = {
  open: boolean;
  title: string;
  message: string;
  onClose: () => void;
};

export function ResultDialog({ open, title, message, onClose }: ResultDialogProps) {
  if (!open || !message) {
    return null;
  }

  return (
    <div className="modal-overlay" role="presentation" onClick={onClose}>
      <section
        className="panel result-modal"
        aria-labelledby="result-title"
        role="dialog"
        aria-modal="true"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="panel-heading">
          <div>
            <p className="section-label">Resultado</p>
            <h2 id="result-title">{title}</h2>
          </div>
        </div>
        <p>{message}</p>
        <button className="primary-action" type="button" onClick={onClose}>
          Fechar
        </button>
      </section>
    </div>
  );
}
