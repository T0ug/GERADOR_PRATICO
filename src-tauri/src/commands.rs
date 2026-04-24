use serde::{Deserialize, Serialize};
use std::fs;

use crate::classifier::classify_document;
use crate::config::AppConfig;
use crate::deduplicator::deduplicate_by_access_key;
use crate::importer::collect_import_candidates;
use crate::parser::{parse_fiscal_document, ParseWarningReason};
use crate::progress::{emit_progress, ProgressStage};
use crate::report::{generate_excel, suggested_report_file_name};
#[derive(Serialize)]
pub struct AppStatus {
    message: String,
}

#[tauri::command]
pub fn app_status() -> AppStatus {
    AppStatus {
        message: "Base React + Tauri iniciada com sucesso.".to_string(),
    }
}

#[tauri::command]
pub fn get_config() -> AppConfig {
    AppConfig::load()
}

#[tauri::command]
pub fn update_config(config: AppConfig) -> Result<(), String> {
    config.save()
}

#[derive(Deserialize)]
pub struct GenerateReportRequest {
    pub cnpj: String,
    pub xml_paths: Vec<String>,
    pub zip_paths: Vec<String>,
    pub export_path: String,
    pub description_mode: String,
    pub word_limit: Option<usize>,
}

#[derive(Serialize)]
pub struct GenerateReportResponse {
    pub success: bool,
    pub file_path: String,
    pub suggested_file_name: String,
    pub entradas_count: usize,
    pub saidas_count: usize,
    pub sem_cnpj_count: usize,
    pub warnings: Vec<String>,
}

#[tauri::command]
pub fn generate_report(
    app: tauri::AppHandle,
    request: GenerateReportRequest,
) -> Result<GenerateReportResponse, String> {
    let mut warnings: Vec<String> = Vec::new();
    let import_total = request.xml_paths.len() + request.zip_paths.len();

    emit_progress(
        &app,
        ProgressStage::Leitura,
        0,
        import_total,
        "Lendo arquivos selecionados...",
    );

    // 1. Importacao de candidatos
    let import_result = collect_import_candidates(&request.xml_paths, &request.zip_paths);
    emit_progress(
        &app,
        ProgressStage::Leitura,
        import_total,
        import_total,
        format!("Leitura concluida: {} XMLs candidatos encontrados.", import_result.candidates.len()),
    );
    for warning in &import_result.warnings {
        warnings.push(format!("{}: {:?}", warning.source_name, warning.reason));
    }

    if import_result.candidates.is_empty() {
        return Err(
            "Nenhum arquivo XML valido foi encontrado nos arquivos informados.".to_string(),
        );
    }

    // 2. Parsing fiscal
    let mut parsed_docs = Vec::new();
    let processing_total = import_result.candidates.len();
    emit_progress(
        &app,
        ProgressStage::Processamento,
        0,
        processing_total,
        "Processando documentos fiscais...",
    );

    for (index, candidate) in import_result.candidates.iter().enumerate() {
        match parse_fiscal_document(&candidate.source_name, &candidate.content) {
            Ok(doc) => parsed_docs.push(doc),
            Err(warning) => {
                if warning.reason != ParseWarningReason::IgnoredEventDocument {
                    warnings.push(format!("{}: {:?}", warning.source_name, warning.reason));
                }
            }
        }

        emit_progress(
            &app,
            ProgressStage::Processamento,
            index + 1,
            processing_total,
            format!("Processando documentos fiscais... {}/{}", index + 1, processing_total.max(1)),
        );
    }

    if parsed_docs.is_empty() {
        return Err(
            "Nenhum documento fiscal valido (NF-e, NFC-e ou CT-e) foi identificado nos XMLs."
                .to_string(),
        );
    }

    // 3. Deduplicacao
    let unique_docs = deduplicate_by_access_key(parsed_docs);

    // 4. Classificacao
    let classified_docs: Vec<_> = unique_docs
        .into_iter()
        .map(|doc| classify_document(doc, &request.cnpj))
        .collect();

    emit_progress(
        &app,
        ProgressStage::Processamento,
        processing_total,
        processing_total,
        format!("Processamento concluido: {} documentos validos.", classified_docs.len()),
    );

    let entradas_count = classified_docs
        .iter()
        .filter(|d| d.classification == crate::classifier::DocumentClassification::Entrada)
        .count();
    let saidas_count = classified_docs
        .iter()
        .filter(|d| d.classification == crate::classifier::DocumentClassification::Saida)
        .count();
    let sem_cnpj_count = classified_docs
        .iter()
        .filter(|d| {
            d.classification == crate::classifier::DocumentClassification::SemCnpjIdentificado
        })
        .count();

    // 5. Geracao de Excel
    let word_limit = if request.description_mode == "limited" {
        request.word_limit
    } else {
        None
    };

    emit_progress(
        &app,
        ProgressStage::Exportacao,
        0,
        1,
        "Gerando arquivo Excel...",
    );
    let file_path = generate_excel(&classified_docs, &request.export_path, word_limit)?;
    let suggested_file_name = suggested_report_file_name(&classified_docs);
    emit_progress(
        &app,
        ProgressStage::Exportacao,
        1,
        1,
        "Exportacao concluida.",
    );

    Ok(GenerateReportResponse {
        success: true,
        file_path,
        suggested_file_name,
        entradas_count,
        saidas_count,
        sem_cnpj_count,
        warnings,
    })
}

#[derive(Deserialize)]
pub struct SaveGeneratedReportRequest {
    pub temp_file_path: String,
    pub destination_path: String,
}

#[tauri::command]
pub fn save_generated_report(request: SaveGeneratedReportRequest) -> Result<String, String> {
    let destination = std::path::Path::new(&request.destination_path);

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    match fs::rename(&request.temp_file_path, &request.destination_path) {
        Ok(_) => Ok(request.destination_path),
        Err(_) => {
            fs::copy(&request.temp_file_path, &request.destination_path)
                .map_err(|error| error.to_string())?;
            let _ = fs::remove_file(&request.temp_file_path);
            Ok(request.destination_path)
        }
    }
}
