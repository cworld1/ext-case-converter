// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{self, ModelRc, SharedString, Model};
use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let app = AppWindow::new()?;

    let app_weak = app.as_weak();
    app.on_drop_files(move |drop_data| {
        let mut files = vec![];
        for line in drop_data.lines() {
            let path = PathBuf::from(line.trim());
            if path.is_file() {
                files.push(path.to_string_lossy().to_string());
            } else if path.is_dir() {
                for entry in WalkDir::new(&path).min_depth(1).max_depth(1) {
                    if let Ok(e) = entry {
                        if e.path().is_file() {
                            files.push(e.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        if let Some(app) = app_weak.upgrade() {
            let shared: Vec<SharedString> = files.into_iter().map(SharedString::from).collect();
            let model: ModelRc<SharedString> = ModelRc::from(&shared[..]);
            app.set_file_list(model);
        }
    });

    let app_weak = app.as_weak();
    app.on_select_files(move || {
        // TODO: 文件选择对话框，暂用空实现
        if let Some(app) = app_weak.upgrade() {
            let shared: Vec<SharedString> = vec![SharedString::from("C:/example.txt")];
            let model: ModelRc<SharedString> = ModelRc::from(&shared[..]);
            app.set_file_list(model);
        }
    });

    let app_weak = app.as_weak();
    app.on_convert_case(move |mode| {
        if let Some(app) = app_weak.upgrade() {
            let files = app.get_file_list();
            let _recursive = app.get_recursive();
            let mut processed = 0;
            let total = files.row_count();
            let mut result = String::new();
            for i in 0..total {
                let file = files.row_data(i).unwrap().to_string();
                let path = PathBuf::from(&file);
                let ext = path.extension().and_then(|e| e.to_str());
                if let Some(ext) = ext {
                    let new_ext = match mode.as_str() {
                        "upper" => ext.to_uppercase(),
                        "lower" => ext.to_lowercase(),
                        _ => ext.to_string(),
                    };
                    let new_path = path.with_extension(new_ext);
                    match fs::rename(&path, &new_path) {
                        Ok(_) => {
                            result += &format!("{} -> {}\n", file, new_path.display());
                        },
                        Err(e) => {
                            result += &format!("{} 失败: {}\n", file, e);
                        }
                    }
                }
                processed += 1;
                app.set_progress(processed as f32 / total.max(1) as f32);
            }
            app.set_result_text(result.into());
        }
    });

    let app_weak = app.as_weak();
    app.on_show_settings(move || {
        let settings = SettingsPanel::new().unwrap();
        let settings_weak = settings.as_weak();
        let app_weak2 = app_weak.clone();
        
        settings.on_close_settings(move || {
            if let Some(app) = app_weak2.upgrade() {
                if let Some(settings) = settings_weak.upgrade() {
                    app.set_language(settings.get_language());
                    app.set_dark_mode(settings.get_dark_mode());
                    settings.hide().unwrap();
                }
            }
        });

        if let Some(app) = app_weak.upgrade() {
            settings.set_language(app.get_language());
            settings.set_dark_mode(app.get_dark_mode());
            settings.show().unwrap();
        }
    });

    let _app_weak = app.as_weak();
    app.on_show_about(move || {
        let about = AboutPanel::new().unwrap();
        let about_weak = about.as_weak();
        
        about.on_close_about(move || {
            if let Some(about) = about_weak.upgrade() {
                about.hide().unwrap();
            }
        });
        
        about.show().unwrap();
    });

    app.run()?;
    Ok(())
}
