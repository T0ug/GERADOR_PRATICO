use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportCandidate {
    pub source_name: String,
    pub origin_path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportWarning {
    pub source_name: String,
    pub reason: ImportWarningReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ImportWarningReason {
    PathNotFound,
    InvalidExtension,
    ReadFailed,
    InvalidZip,
    InvalidUtf8,
    ZipEntryReadFailed,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct ImportResult {
    pub candidates: Vec<ImportCandidate>,
    pub warnings: Vec<ImportWarning>,
}

pub fn collect_import_candidates(xml_paths: &[String], zip_paths: &[String]) -> ImportResult {
    let mut result = ImportResult::default();

    for path in xml_paths {
        collect_direct_xml(Path::new(path), &mut result);
    }

    for path in zip_paths {
        collect_zip_xmls(Path::new(path), &mut result);
    }

    result
}

fn collect_direct_xml(path: &Path, result: &mut ImportResult) {
    if !path.exists() {
        push_warning(path, result, ImportWarningReason::PathNotFound);
        return;
    }

    if !has_extension(path, "xml") {
        push_warning(path, result, ImportWarningReason::InvalidExtension);
        return;
    }

    match std::fs::read_to_string(path) {
        Ok(content) => result.candidates.push(ImportCandidate {
            source_name: source_name(path),
            origin_path: path.display().to_string(),
            content,
        }),
        Err(error) if error.kind() == std::io::ErrorKind::InvalidData => {
            push_warning(path, result, ImportWarningReason::InvalidUtf8);
        }
        Err(_) => push_warning(path, result, ImportWarningReason::ReadFailed),
    }
}

fn collect_zip_xmls(path: &Path, result: &mut ImportResult) {
    if !path.exists() {
        push_warning(path, result, ImportWarningReason::PathNotFound);
        return;
    }

    if !has_extension(path, "zip") {
        push_warning(path, result, ImportWarningReason::InvalidExtension);
        return;
    }

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            push_warning(path, result, ImportWarningReason::ReadFailed);
            return;
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(_) => {
            push_warning(path, result, ImportWarningReason::InvalidZip);
            return;
        }
    };

    for index in 0..archive.len() {
        let mut entry = match archive.by_index(index) {
            Ok(entry) => entry,
            Err(_) => {
                push_warning(path, result, ImportWarningReason::ZipEntryReadFailed);
                continue;
            }
        };

        if entry.is_dir() || !entry.name().to_ascii_lowercase().ends_with(".xml") {
            continue;
        }

        let entry_name = entry.name().to_string();
        let mut content = String::new();

        match entry.read_to_string(&mut content) {
            Ok(_) => result.candidates.push(ImportCandidate {
                source_name: format!("{}::{}", source_name(path), entry_name),
                origin_path: format!("{}::{}", path.display(), entry_name),
                content,
            }),
            Err(error) if error.kind() == std::io::ErrorKind::InvalidData => {
                result.warnings.push(ImportWarning {
                    source_name: format!("{}::{}", source_name(path), entry_name),
                    reason: ImportWarningReason::InvalidUtf8,
                });
            }
            Err(_) => result.warnings.push(ImportWarning {
                source_name: format!("{}::{}", source_name(path), entry_name),
                reason: ImportWarningReason::ZipEntryReadFailed,
            }),
        }
    }
}

fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}

fn source_name(path: &Path) -> String {
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

fn push_warning(path: &Path, result: &mut ImportResult, reason: ImportWarningReason) {
    result.warnings.push(ImportWarning {
        source_name: source_name(path),
        reason,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use zip::write::SimpleFileOptions;

    #[test]
    fn returns_direct_xml_as_candidate() {
        let temp_dir = TestDir::new("direct_xml");
        let xml_path = temp_dir.path().join("nota.xml");
        fs::write(&xml_path, "<nota><numero>1</numero></nota>").unwrap();

        let result = collect_import_candidates(&[path_string(&xml_path)], &[]);

        assert_eq!(result.warnings, Vec::new());
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.candidates[0].source_name, "nota.xml");
        assert_eq!(result.candidates[0].content, "<nota><numero>1</numero></nota>");
    }

    #[test]
    fn returns_nested_xml_inside_zip_as_candidate() {
        let temp_dir = TestDir::new("zip_xml");
        let zip_path = temp_dir.path().join("notas.zip");
        create_zip_with_xml(&zip_path, "internas/abril/nota.xml", "<cte>ok</cte>");

        let result = collect_import_candidates(&[], &[path_string(&zip_path)]);

        assert_eq!(result.warnings, Vec::new());
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.candidates[0].source_name, "notas.zip::internas/abril/nota.xml");
        assert_eq!(result.candidates[0].content, "<cte>ok</cte>");
    }

    #[test]
    fn invalid_direct_path_generates_warning_without_stopping() {
        let temp_dir = TestDir::new("invalid_path");
        let missing_path = temp_dir.path().join("ausente.xml");

        let result = collect_import_candidates(&[path_string(&missing_path)], &[]);

        assert_eq!(result.candidates, Vec::new());
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0].source_name, "ausente.xml");
        assert_eq!(result.warnings[0].reason, ImportWarningReason::PathNotFound);
    }

    #[test]
    fn corrupt_zip_generates_warning_without_stopping() {
        let temp_dir = TestDir::new("corrupt_zip");
        let zip_path = temp_dir.path().join("quebrado.zip");
        fs::write(&zip_path, "conteudo sem estrutura zip").unwrap();

        let result = collect_import_candidates(&[], &[path_string(&zip_path)]);

        assert_eq!(result.candidates, Vec::new());
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0].source_name, "quebrado.zip");
        assert_eq!(result.warnings[0].reason, ImportWarningReason::InvalidZip);
    }

    fn create_zip_with_xml(zip_path: &Path, entry_name: &str, content: &str) {
        let file = File::create(zip_path).unwrap();
        let mut writer = zip::ZipWriter::new(file);

        writer
            .start_file(entry_name, SimpleFileOptions::default())
            .unwrap();
        writer.write_all(content.as_bytes()).unwrap();
        writer.finish().unwrap();
    }

    fn path_string(path: &Path) -> String {
        path.display().to_string()
    }

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(name: &str) -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "gerador_relatorio_notas_{name}_{}_{}",
                std::process::id(),
                timestamp
            ));

            fs::create_dir_all(&path).unwrap();

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
