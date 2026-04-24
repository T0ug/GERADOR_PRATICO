use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub last_cnpj: Option<String>,
    pub last_import_dir: Option<String>,
    pub last_export_dir: Option<String>,
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(Path::to_path_buf))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
            .join("config.json")
    }

    pub fn load() -> Self {
        Self::load_from_path(&Self::config_path())
    }

    pub fn save(&self) -> Result<(), String> {
        self.save_to_path(&Self::config_path())
    }

    fn load_from_path(path: &Path) -> Self {
        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    fn save_to_path(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }

        let content = serde_json::to_string_pretty(self).map_err(|error| error.to_string())?;
        fs::write(path, content).map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn load_returns_default_when_file_does_not_exist() {
        let temp_dir = TestDir::new("missing_config");
        let path = temp_dir.path().join("config.json");

        let config = AppConfig::load_from_path(&path);

        assert_eq!(config, AppConfig::default());
    }

    #[test]
    fn save_and_load_roundtrip_config() {
        let temp_dir = TestDir::new("roundtrip_config");
        let path = temp_dir.path().join("config.json");
        let expected = AppConfig {
            last_cnpj: Some("12.345.678/0001-90".to_string()),
            last_import_dir: Some("C:\\documentos\\xml".to_string()),
            last_export_dir: Some("C:\\documentos\\saida".to_string()),
        };

        expected.save_to_path(&path).unwrap();
        let loaded = AppConfig::load_from_path(&path);

        assert_eq!(loaded, expected);
    }

    #[test]
    fn load_returns_default_when_json_is_invalid() {
        let temp_dir = TestDir::new("invalid_json");
        let path = temp_dir.path().join("config.json");
        fs::write(&path, "{ invalid json").unwrap();

        let config = AppConfig::load_from_path(&path);

        assert_eq!(config, AppConfig::default());
    }

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(label: &str) -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "gerador_relatorio_notas_{label}_{}_{}",
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
