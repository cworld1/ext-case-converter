use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub language: String,
    pub theme: String,
    pub dark_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "zh".to_string(),
            theme: "system".to_string(),
            dark_mode: false,
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
        if let Some(theme) = table.get("theme").and_then(|v| v.as_str()) {
            config.theme = theme.to_string();
        }
        if let Some(dark_mode) = table.get("dark_mode").and_then(|v| v.as_bool()) {
            config.dark_mode = dark_mode;
        }
        
        Ok(config)
    }

    fn to_toml(&self) -> String {
        format!(
            r#"# File Extension Case Converter Configuration
language = "{}"
theme = "{}"
dark_mode = {}
"#,
            self.language, self.theme, self.dark_mode
        )
    }

    pub fn get_theme_index(&self) -> i32 {
        match self.theme.as_str() {
            "light" => 0,
            "dark" => 1,
            "system" => 2,
            _ => 2,
        }
    }

    pub fn set_theme_by_index(&mut self, index: i32) {
        match index {
            0 => {
                self.theme = "light".to_string();
                self.dark_mode = false;
            },
            1 => {
                self.theme = "dark".to_string();
                self.dark_mode = true;
            },
            _ => {
                self.theme = "system".to_string();
                self.dark_mode = false;
            }
        }
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
