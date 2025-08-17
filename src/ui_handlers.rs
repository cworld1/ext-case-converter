use slint::{ModelRc, SharedString, Model, ComponentHandle};
use std::rc::Rc;
use std::cell::RefCell;

use crate::config::AppConfig;
use crate::file_handler;
use crate::{AppWindow, SettingsPanel, AboutPanel};

pub fn setup_file_operations(app: &AppWindow, _config: Rc<RefCell<AppConfig>>) {
    let app_weak = app.as_weak();
    app.on_drop_files(move |drop_data| {
        let file_paths: Vec<String> = drop_data.lines()
            .map(|line| line.trim().to_string())
            .collect();
        
        if let Some(app) = app_weak.upgrade() {
            let shared: Vec<SharedString> = file_paths.into_iter().map(SharedString::from).collect();
            let model: ModelRc<SharedString> = ModelRc::from(&shared[..]);
            app.set_file_list(model);
        }
    });

    let app_weak = app.as_weak();
    app.on_select_files(move || {
        if let Some(app) = app_weak.upgrade() {
            if let Some(file_paths) = file_handler::select_files() {
                let files_list: Vec<String> = file_paths.into_iter()
                    .map(|path| path.to_string_lossy().to_string())
                    .collect();
                
                if !files_list.is_empty() {
                    let shared: Vec<SharedString> = files_list.into_iter().map(SharedString::from).collect();
                    let model: ModelRc<SharedString> = ModelRc::from(&shared[..]);
                    app.set_file_list(model);
                }
            }
        }
    });

    let app_weak = app.as_weak();
    app.on_convert_case(move |mode| {
        if let Some(app) = app_weak.upgrade() {
            let files = app.get_file_list();
            let recursive = app.get_recursive();
            let mut result = String::new();
            
            // 收集文件路径
            let file_paths: Vec<String> = (0..files.row_count())
                .map(|i| files.row_data(i).unwrap().to_string())
                .collect();
            
            // 使用 file_handler 模块收集所有文件
            let all_files = file_handler::collect_files_from_paths(&file_paths, recursive);
            
            let total = all_files.len();
            app.set_progress(0.0);
            
            for (index, path) in all_files.iter().enumerate() {
                let result_text = match file_handler::convert_file_extension_case(path, mode.as_str() == "upper") {
                    Ok(new_path) => {
                        if new_path != *path {
                            format!("{} -> {}\n", path.display(), new_path.display())
                        } else {
                            format!("{} (无需更改)\n", path.display())
                        }
                    },
                    Err(e) => format!("{} 失败: {}\n", path.display(), e),
                };
                result += &result_text;
                
                let progress = (index + 1) as f32 / total.max(1) as f32;
                app.set_progress(progress);
            }
            
            if total == 0 {
                result = "没有找到可处理的文件。\n".to_string();
            }
            
            app.set_result_text(result.into());
        }
    });
}

pub fn setup_settings(app: &AppWindow, config: Rc<RefCell<AppConfig>>) {
    let app_weak = app.as_weak();
    let config_clone = config.clone();
    app.on_show_settings(move || {
        let settings = SettingsPanel::new().unwrap();
        let settings_weak = settings.as_weak();
        let app_weak2 = app_weak.clone();
        let config_clone2 = config_clone.clone();
        
        // 设置当前配置值
        {
            let cfg = config_clone.borrow();
            settings.set_language(cfg.language.clone().into());
            settings.set_dark_mode(cfg.dark_mode);
            settings.set_selected_language_index(cfg.get_language_index());
            settings.set_selected_theme_index(cfg.get_theme_index());
        }
        
        settings.on_close_settings(move || {
            if let Some(app) = app_weak2.upgrade() {
                if let Some(settings) = settings_weak.upgrade() {
                    let mut cfg = config_clone2.borrow_mut();
                    
                    // 更新配置
                    cfg.set_language_by_index(settings.get_selected_language_index());
                    cfg.set_theme_by_index(settings.get_selected_theme_index());
                    
                    // 保存配置
                    if let Err(e) = cfg.save() {
                        eprintln!("Failed to save config: {}", e);
                    }
                    
                    // 设置 Slint 的语言
                    let _ = match cfg.language.as_str() {
                        "zh" => slint::select_bundled_translation("zh_CN"),
                        "en" => slint::select_bundled_translation("en_US"),
                        _ => slint::select_bundled_translation("zh_CN"),
                    };
                    
                    // 应用设置到主窗口
                    app.set_language(cfg.language.clone().into());
                    app.set_dark_mode(cfg.dark_mode);
                    
                    settings.hide().unwrap();
                }
            }
        });

        settings.show().unwrap();
    });
}

pub fn setup_about(app: &AppWindow) {
    let app_weak = app.as_weak();
    app.on_show_about(move || {
        if let Some(app) = app_weak.upgrade() {
            let about = AboutPanel::new().unwrap();
            let about_weak = about.as_weak();
            
            about.on_close_about(move || {
                if let Some(about) = about_weak.upgrade() {
                    about.hide().unwrap();
                }
            });
            
            about.show().unwrap();
        }
    });
}
