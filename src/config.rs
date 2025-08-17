use std::fs;
use std::path::PathBuf;

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
        // 使用程序目录下的config.txt
        let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop(); // 移除exe文件名
        path.push("config.txt");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_file_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                return Self::parse_config(&content);
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_file_path();
        let content = format!(
            "language={}\ntheme={}\ndark_mode={}\n",
            self.language, self.theme, self.dark_mode
        );
        fs::write(&path, content)?;
        Ok(())
    }

    fn parse_config(content: &str) -> Self {
        let mut config = Self::default();
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "language" => config.language = value.trim().to_string(),
                    "theme" => config.theme = value.trim().to_string(),
                    "dark_mode" => config.dark_mode = value.trim() == "true",
                    _ => {}
                }
            }
        }
        config
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
