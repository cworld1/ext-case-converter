#[derive(Clone)]
pub struct I18n {
    current_language: String,
}

impl I18n {
    pub fn new() -> Self {
        Self {
            current_language: "zh".to_string(),
        }
    }
    
    pub fn set_language(&mut self, language: &str) {
        self.current_language = language.to_string();
    }
    
    pub fn get(&self, key: &str) -> String {
        match (self.current_language.as_str(), key) {
            // 中文翻译
            ("zh", "app_title") => "扩展名大小写转换工具".to_string(),
            ("zh", "select_files") => "选择文件/文件夹".to_string(),
            ("zh", "recursive_dirs") => "递归子目录".to_string(),
            ("zh", "settings") => "设置".to_string(),
            ("zh", "about") => "关于".to_string(),
            ("zh", "drag_drop_hint") => "将文件或文件夹拖放到此处".to_string(),
            ("zh", "convert_upper") => "转为大写".to_string(),
            ("zh", "convert_lower") => "转为小写".to_string(),
            ("zh", "language") => "语言".to_string(),
            ("zh", "theme") => "主题".to_string(),
            ("zh", "theme_light") => "浅色".to_string(),
            ("zh", "theme_dark") => "深色".to_string(),
            ("zh", "theme_system") => "跟随系统".to_string(),
            ("zh", "ok") => "确定".to_string(),
            ("zh", "cancel") => "取消".to_string(),
            ("zh", "failed") => "失败".to_string(),
            
            // 英文翻译
            ("en", "app_title") => "File Extension Case Converter".to_string(),
            ("en", "select_files") => "Select Files/Folders".to_string(),
            ("en", "recursive_dirs") => "Recursive Subdirectories".to_string(),
            ("en", "settings") => "Settings".to_string(),
            ("en", "about") => "About".to_string(),
            ("en", "drag_drop_hint") => "Drag and drop files or folders here".to_string(),
            ("en", "convert_upper") => "Convert to Uppercase".to_string(),
            ("en", "convert_lower") => "Convert to Lowercase".to_string(),
            ("en", "language") => "Language".to_string(),
            ("en", "theme") => "Theme".to_string(),
            ("en", "theme_light") => "Light".to_string(),
            ("en", "theme_dark") => "Dark".to_string(),
            ("en", "theme_system") => "Follow System".to_string(),
            ("en", "ok") => "OK".to_string(),
            ("en", "cancel") => "Cancel".to_string(),
            ("en", "failed") => "Failed".to_string(),
            
            // 默认返回键名
            _ => key.to_string(),
        }
    }
    
    pub fn get_language_names(&self) -> Vec<String> {
        vec!["中文".to_string(), "English".to_string()]
    }
    
    pub fn get_theme_names(&self) -> Vec<String> {
        vec![
            self.get("theme_light"),
            self.get("theme_dark"),
            self.get("theme_system"),
        ]
    }
}
