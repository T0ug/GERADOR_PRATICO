use serde::Serialize;
use tauri::{AppHandle, Emitter};

pub const PROGRESS_EVENT: &str = "report-progress";

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub enum ProgressStage {
    Leitura,
    Processamento,
    Exportacao,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ProgressUpdate {
    pub stage: ProgressStage,
    pub current: usize,
    pub total: usize,
    pub percent: u8,
    pub message: String,
}

impl ProgressUpdate {
    pub fn new(stage: ProgressStage, current: usize, total: usize, message: impl Into<String>) -> Self {
        let normalized_total = total.max(1);
        let bounded_current = current.min(normalized_total);
        let percent = ((bounded_current * 100) / normalized_total) as u8;

        Self {
            stage,
            current: bounded_current,
            total: normalized_total,
            percent,
            message: message.into(),
        }
    }
}

pub fn emit_progress(
    app: &AppHandle,
    stage: ProgressStage,
    current: usize,
    total: usize,
    message: impl Into<String>,
) {
    let _ = app.emit(PROGRESS_EVENT, ProgressUpdate::new(stage, current, total, message));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_update_calculates_percent() {
        let update = ProgressUpdate::new(ProgressStage::Processamento, 3, 4, "Processando");

        assert_eq!(update.percent, 75);
        assert_eq!(update.current, 3);
        assert_eq!(update.total, 4);
    }

    #[test]
    fn progress_update_handles_zero_total() {
        let update = ProgressUpdate::new(ProgressStage::Leitura, 0, 0, "Lendo");

        assert_eq!(update.percent, 0);
        assert_eq!(update.current, 0);
        assert_eq!(update.total, 1);
    }
}
