use crate::classifier::{ClassifiedDocument, DocumentClassification};
use chrono::{DateTime, Datelike};
use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Image, Workbook, Worksheet};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;

pub struct ReportSheetNames {
    pub entradas: &'static str,
    pub saidas: &'static str,
    pub sem_cnpj_identificado: &'static str,
}

pub const REPORT_SHEETS: ReportSheetNames = ReportSheetNames {
    entradas: "Entradas",
    saidas: "Saidas",
    sem_cnpj_identificado: "Notas sem CNPJ identificado",
};

const LOGO_WIDTH_PIXELS: u32 = 240;
const LOGO_HEIGHT_PIXELS: u32 = 80;
const HEADER_ROW: u32 = 4;
const REPORT_LOGO_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../icones_e_logo/LOGO_alta_resolução.png"));

pub fn generate_excel(
    documents: &[ClassifiedDocument],
    export_path: &str,
    description_word_limit: Option<usize>,
) -> Result<String, String> {
    let final_path = determine_final_path(documents, export_path)?;

    let mut workbook = Workbook::new();

    let data_text_format = Format::new().set_border(FormatBorder::Thin);
    let data_currency_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_num_format(r#"_-"R$"* #,##0.00_-;_-"R$"* -#,##0.00_-;_-"R$"* "-"??_-;_-@_-"#);

    let base_headers: Vec<&str> = vec![
        "Data",
        "Numero da nota",
        "Valor",
        "CFOP",
        "Descricao dos itens",
    ];

    let entradas: Vec<_> = documents
        .iter()
        .filter(|d| d.classification == DocumentClassification::Entrada)
        .collect();
    let saidas: Vec<_> = documents
        .iter()
        .filter(|d| d.classification == DocumentClassification::Saida)
        .collect();
    let sem_cnpj: Vec<_> = documents
        .iter()
        .filter(|d| d.classification == DocumentClassification::SemCnpjIdentificado)
        .collect();

    let entradas_extra_headers: Vec<&str> = vec!["Emitente", "Remetente"];
    let saidas_extra_headers: Vec<&str> = vec!["Destinatario"];
    let sem_cnpj_extra_headers: Vec<&str> = vec!["Tomador", "Destinatario", "Remetente"];

    #[derive(Clone, Copy)]
    enum SheetType {
        Entradas,
        Saidas,
        SemCnpj,
    }

    let groups: Vec<(&str, Vec<&ClassifiedDocument>, Vec<&str>, SheetType)> = vec![
        (
            REPORT_SHEETS.entradas,
            entradas,
            entradas_extra_headers,
            SheetType::Entradas,
        ),
        (
            REPORT_SHEETS.saidas,
            saidas,
            saidas_extra_headers,
            SheetType::Saidas,
        ),
        (
            REPORT_SHEETS.sem_cnpj_identificado,
            sem_cnpj,
            sem_cnpj_extra_headers,
            SheetType::SemCnpj,
        ),
    ];

    for (sheet_name, docs, extra_headers, sheet_type) in groups {
        let worksheet = workbook
            .add_worksheet()
            .set_name(sheet_name)
            .map_err(|e| e.to_string())?;

        let all_headers: Vec<&str> = base_headers
            .iter()
            .copied()
            .chain(extra_headers.iter().copied())
            .collect();

        configure_sheet_layout(worksheet, &all_headers)?;
        insert_logo_above_header(worksheet, &all_headers)?;

        for (col, header) in all_headers.iter().enumerate() {
            let header_format = build_header_format(col, all_headers.len());
            worksheet
                .write_string_with_format(HEADER_ROW, col as u16, *header, &header_format)
                .map_err(|e| e.to_string())?;
        }

        let mut row = HEADER_ROW + 1;
        for doc in docs {
            let date_str = doc
                .document
                .issue_date
                .as_deref()
                .map(format_issue_date)
                .unwrap_or_default();
            worksheet
                .write_string_with_format(row, 0, &date_str, &data_text_format)
                .map_err(|e| e.to_string())?;

            worksheet
                .write_string_with_format(
                    row,
                    1,
                    &doc.document.document_number,
                    &data_text_format,
                )
                .map_err(|e| e.to_string())?;

            if let Some(val_str) = &doc.document.total_value {
                let val: f64 = val_str.replace(",", ".").parse().unwrap_or(0.0);
                worksheet
                    .write_number_with_format(row, 2, val, &data_currency_format)
                    .map_err(|e| e.to_string())?;
            } else {
                worksheet
                    .write_blank(row, 2, &data_currency_format)
                    .map_err(|e| e.to_string())?;
            }

            let cfops = join_unique_values(&doc.document.cfops);
            worksheet
                .write_string_with_format(row, 3, &cfops, &data_text_format)
                .map_err(|e| e.to_string())?;

            let limited_descriptions: Vec<String> = doc
                .document
                .descriptions
                .iter()
                .map(|description| limit_description(description, description_word_limit))
                .collect();
            let descriptions = limited_descriptions.join("; ");
            worksheet
                .write_string_with_format(row, 4, &descriptions, &data_text_format)
                .map_err(|e| e.to_string())?;

            match sheet_type {
                SheetType::Entradas => {
                    write_party_cell(worksheet, row, 5, doc.document.issuer.as_ref(), &data_text_format)?;
                    write_party_cell(worksheet, row, 6, doc.document.sender.as_ref(), &data_text_format)?;
                }
                SheetType::Saidas => {
                    write_party_cell(
                        worksheet,
                        row,
                        5,
                        doc.document.recipient.as_ref(),
                        &data_text_format,
                    )?;
                }
                SheetType::SemCnpj => {
                    write_party_cell(worksheet, row, 5, doc.document.taker.as_ref(), &data_text_format)?;
                    write_party_cell(
                        worksheet,
                        row,
                        6,
                        doc.document.recipient.as_ref(),
                        &data_text_format,
                    )?;
                    write_party_cell(worksheet, row, 7, doc.document.sender.as_ref(), &data_text_format)?;
                }
            }

            row += 1;
        }
    }

    workbook.save(&final_path).map_err(|e| e.to_string())?;

    Ok(final_path)
}

fn configure_sheet_layout(worksheet: &mut Worksheet, headers: &[&str]) -> Result<(), String> {
    worksheet
        .set_row_height_pixels(0, 72)
        .map_err(|e| e.to_string())?;
    worksheet
        .set_row_height_pixels(1, 12)
        .map_err(|e| e.to_string())?;
    worksheet
        .set_row_height_pixels(2, 12)
        .map_err(|e| e.to_string())?;
    worksheet
        .set_row_height_pixels(3, 12)
        .map_err(|e| e.to_string())?;

    for (col, _) in headers.iter().enumerate() {
        worksheet
            .set_column_width(col as u16, default_column_width(col))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn insert_logo_above_header(worksheet: &mut Worksheet, headers: &[&str]) -> Result<(), String> {
    let total_width_pixels: u32 = headers
        .iter()
        .enumerate()
        .map(|(col, _)| width_to_pixels(default_column_width(col)))
        .sum();

    let image = Image::new_from_buffer(REPORT_LOGO_BYTES)
        .map_err(|e| e.to_string())?
        .set_scale_to_size(LOGO_WIDTH_PIXELS, LOGO_HEIGHT_PIXELS, true);

    let scaled_width = image.width().min(LOGO_WIDTH_PIXELS as f64).round() as u32;
    let x_offset = total_width_pixels.saturating_sub(scaled_width) / 2;

    worksheet
        .insert_image_with_offset(0, 0, &image, x_offset, 4)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn build_header_format(col_index: usize, total_columns: usize) -> Format {
    let mut format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border_top(FormatBorder::Thick)
        .set_border_bottom(FormatBorder::Thick)
        .set_border_left(FormatBorder::Thin)
        .set_border_right(FormatBorder::Thin);

    if col_index == 0 {
        format = format.set_border_left(FormatBorder::Thick);
    }

    if col_index + 1 == total_columns {
        format = format.set_border_right(FormatBorder::Thick);
    }

    format
}

fn write_party_cell(
    worksheet: &mut Worksheet,
    row: u32,
    col: u16,
    party: Option<&crate::parser::FiscalParty>,
    format: &Format,
) -> Result<(), String> {
    if let Some(party) = party {
        let formatted = format!("{} {}", party.name, party.document);
        worksheet
            .write_string_with_format(row, col, &formatted, format)
            .map_err(|e| e.to_string())?;
    } else {
        worksheet
            .write_blank(row, col, format)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn default_column_width(col: usize) -> f64 {
    match col {
        0 => 20.0,
        1 => 15.0,
        2 => 15.0,
        3 => 10.0,
        4 => 50.0,
        _ => 30.0,
    }
}

fn join_unique_values(values: &[String]) -> String {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for value in values {
        if seen.insert(value.as_str()) {
            unique.push(value.as_str());
        }
    }

    unique.join(";")
}

fn limit_description(description: &str, description_word_limit: Option<usize>) -> String {
    if let Some(limit) = description_word_limit {
        let words: Vec<&str> = description.split_whitespace().collect();
        if words.len() > limit {
            words.into_iter().take(limit).collect::<Vec<&str>>().join(" ")
        } else {
            description.to_string()
        }
    } else {
        description.to_string()
    }
}

fn width_to_pixels(width: f64) -> u32 {
    if width < 1.0 {
        (width * 12.0 + 0.5).round() as u32
    } else {
        (width * 7.0 + 5.0).round() as u32
    }
}

fn determine_final_path(
    documents: &[ClassifiedDocument],
    base_path: &str,
) -> Result<String, String> {
    let mut months = HashSet::new();
    let mut unique_months = Vec::new();

    for doc in documents {
        if let Some(date_str) = &doc.document.issue_date {
            if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
                let month_index = dt.month() as usize;
                if !months.contains(&month_index) && month_index >= 1 && month_index <= 12 {
                    months.insert(month_index);
                    unique_months.push(month_index);
                }
            } else if let Ok(dt) = date_str.parse::<chrono::NaiveDate>() {
                let month_index = dt.month() as usize;
                if !months.contains(&month_index) && month_index >= 1 && month_index <= 12 {
                    months.insert(month_index);
                    unique_months.push(month_index);
                }
            } else if let Ok(dt) = date_str.parse::<chrono::NaiveDateTime>() {
                let month_index = dt.month() as usize;
                if !months.contains(&month_index) && month_index >= 1 && month_index <= 12 {
                    months.insert(month_index);
                    unique_months.push(month_index);
                }
            }
        }
    }

    unique_months.sort();

    let month_names = [
        "",
        "Janeiro",
        "Fevereiro",
        "Marco",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];

    let months_str = unique_months
        .into_iter()
        .filter_map(|idx| {
            if idx <= 12 {
                Some(month_names[idx])
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join(", ");

    let file_name = build_report_file_name(&months_str);

    let export_path_obj = Path::new(base_path);
    let has_xlsx_name = export_path_obj
        .extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| extension.eq_ignore_ascii_case("xlsx"));

    let final_path = if has_xlsx_name {
        export_path_obj.to_path_buf()
    } else {
        let parent = if base_path.trim().is_empty() {
            std::env::temp_dir()
        } else {
            export_path_obj.to_path_buf()
        };

        parent.join(file_name)
    };

    Ok(final_path.to_string_lossy().to_string())
}

pub fn suggested_report_file_name(documents: &[ClassifiedDocument]) -> String {
    let months_str = collect_report_months(documents);
    build_report_file_name(&months_str)
}

fn collect_report_months(documents: &[ClassifiedDocument]) -> String {
    let mut months = HashSet::new();
    let mut unique_months = Vec::new();

    for doc in documents {
        if let Some(date_str) = &doc.document.issue_date {
            if let Some(date) = parse_issue_date(date_str) {
                let month_index = date.month() as usize;
                if !months.contains(&month_index) && month_index >= 1 && month_index <= 12 {
                    months.insert(month_index);
                    unique_months.push(month_index);
                }
            }
        }
    }

    unique_months.sort();

    let month_names = [
        "",
        "Janeiro",
        "Fevereiro",
        "Marco",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];

    unique_months
        .into_iter()
        .filter_map(|idx| {
            if idx <= 12 {
                Some(month_names[idx])
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join(", ")
}

fn build_report_file_name(months_str: &str) -> String {
    if months_str.is_empty() {
        "Relatorio de notas.xlsx".to_string()
    } else {
        format!("Relatorio de notas [{}].xlsx", months_str)
    }
}

fn format_issue_date(date_str: &str) -> String {
    parse_issue_date(date_str)
        .map(|date| date.format("%d/%m/%Y").to_string())
        .unwrap_or_default()
}

fn parse_issue_date(date_str: &str) -> Option<chrono::NaiveDate> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        Some(dt.date_naive())
    } else if let Ok(dt) = date_str.parse::<chrono::NaiveDate>() {
        Some(dt)
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        Some(dt.date())
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S") {
        Some(dt.date())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classifier::DocumentClassification;
    use crate::parser::{FiscalDocumentType, FiscalParty, ParsedFiscalDocument};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn tests_generate_excel_and_name_formatting() {
        let doc1 = ClassifiedDocument {
            classification: DocumentClassification::Entrada,
            document: ParsedFiscalDocument {
                source_name: "nfe.xml".to_string(),
                access_key: "111".to_string(),
                document_type: FiscalDocumentType::Nfe,
                issue_date: Some("2026-04-10T10:00:00-03:00".to_string()),
                document_number: "1".to_string(),
                total_value: Some("150.50".to_string()),
                cfops: vec!["5102".to_string(), "5102".to_string()],
                descriptions: vec!["Lapis novo".to_string(), "Caneta velha azul".to_string()],
                issuer: Some(FiscalParty {
                    name: "Fornecedor".to_string(),
                    document: "1111".to_string(),
                }),
                taker: None,
                recipient: None,
                sender: None,
            },
        };

        let doc2 = ClassifiedDocument {
            classification: DocumentClassification::Saida,
            document: ParsedFiscalDocument {
                source_name: "nfe2.xml".to_string(),
                access_key: "222".to_string(),
                document_type: FiscalDocumentType::Nfe,
                issue_date: Some("2026-05-10T10:00:00-03:00".to_string()),
                document_number: "2".to_string(),
                total_value: Some("10.00".to_string()),
                cfops: vec!["5102".to_string()],
                descriptions: vec!["Borracha grande de teste de uso continuo".to_string()],
                issuer: None,
                taker: None,
                recipient: None,
                sender: None,
            },
        };

        let doc3 = ClassifiedDocument {
            classification: DocumentClassification::SemCnpjIdentificado,
            document: ParsedFiscalDocument {
                source_name: "nfe3.xml".to_string(),
                access_key: "333".to_string(),
                document_type: FiscalDocumentType::Nfe,
                issue_date: None,
                document_number: "3".to_string(),
                total_value: None,
                cfops: vec![],
                descriptions: vec![],
                issuer: None,
                taker: None,
                recipient: None,
                sender: None,
            },
        };

        let temp_dir = std::env::temp_dir().join(format!(
            "gerador_relatorio_notas_report_test_{}_{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();
        let export_base = temp_dir.to_string_lossy().to_string();
        let docs = vec![doc1, doc2, doc3];

        let result = generate_excel(&docs, &export_base, Some(2));
        assert!(result.is_ok());
        let final_path = result.unwrap();

        assert!(final_path.contains("Relatorio de notas [Abril, Maio].xlsx"));
        assert!(Path::new(&final_path).exists());

        let _ = fs::remove_file(&final_path);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn formats_issue_date_as_brazilian_day_month_year() {
        assert_eq!(format_issue_date("2026-01-03T18:59:00-03:00"), "03/01/2026");
        assert_eq!(format_issue_date("2026-01-03"), "03/01/2026");
        assert_eq!(format_issue_date("2026-01-03 18:59:00"), "03/01/2026");
    }

    #[test]
    fn joins_unique_cfops_preserving_order() {
        let values = vec![
            "5405".to_string(),
            "5405".to_string(),
            "5102".to_string(),
            "5405".to_string(),
            "6101".to_string(),
        ];

        assert_eq!(join_unique_values(&values), "5405;5102;6101");
    }

    #[test]
    fn embedded_report_logo_is_available() {
        assert!(!REPORT_LOGO_BYTES.is_empty());
    }
}
