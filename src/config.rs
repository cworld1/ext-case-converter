use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub language: String,
    pub default_recursive: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "zh".to_string(),
            default_recursive: false,
        }
    }
}

impl AppConfig {
    pub fn config_file_path() -> PathBuf {
        let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop(); // 移除exe文件名
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_file_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                return Self::parse_toml(&content).unwrap_or_default();
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_file_path();
        let content = self.to_toml();
        fs::write(&path, content)?;
        Ok(())
    }

    fn parse_toml(content: &str) -> Result<Self, toml::de::Error> {
        let table: HashMap<String, toml::Value> = toml::from_str(content)?;
        
        let mut config = Self::default();
        if let Some(language) = table.get("language").and_then(|v| v.as_str()) {
            config.language = language.to_string();
        }
        if let Some(default_recursive) = table.get("default_recursive").and_then(|v| v.as_bool()) {
            config.default_recursive = default_recursive;
        }
        
        Ok(config)
    }

    fn to_toml(&self) -> String {
        format!(
            r#"# File Extension Case Converter Configuration
language = "{}"
default_recursive = {}
"#,
            self.language, self.default_recursive
        )
    }

    pub fn get_language_index(&self) -> i32 {
        match self.language.as_str() {
            "zh" => 0,
            "en" => 1,
            _ => 0,
        }
    }

    pub fn set_language_by_index(&mut self, index: i32) {
        match index {
            0 => self.language = "zh".to_string(),
            1 => self.language = "en".to_string(),
            _ => self.language = "zh".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_locale(&self) -> &str {
        match self.language.as_str() {
            "zh" => "zh_CN",
            "en" => "en_US",
            _ => "zh_CN",
        }
    }
}
