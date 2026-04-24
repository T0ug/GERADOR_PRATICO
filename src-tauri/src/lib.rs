pub mod classifier;
pub mod commands;
pub mod config;
pub mod deduplicator;
pub mod errors;
pub mod importer;
pub mod parser;
pub mod progress;
pub mod report;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::app_status,
            commands::get_config,
            commands::update_config,
            commands::generate_report,
            commands::save_generated_report
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o aplicativo Tauri");
}
