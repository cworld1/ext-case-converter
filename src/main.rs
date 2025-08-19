// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::ComponentHandle;
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

mod config;
mod file_handler;
mod ui_handlers;

use config::AppConfig;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let config = Rc::new(RefCell::new(AppConfig::load()));
    let app = AppWindow::new()?;
    
    // 应用初始配置并设置语言
    {
        let cfg = config.borrow();
        app.set_language(cfg.language.clone().into());
        app.set_recursive(cfg.default_recursive);
        
        // 设置 Slint 的语言
        let _ = match cfg.language.as_str() {
            "zh" => slint::select_bundled_translation("zh_CN"),
            "en" => slint::select_bundled_translation("en_US"),
            _ => slint::select_bundled_translation("zh_CN"),
        };
    }

    // 设置事件处理器
    ui_handlers::setup_file_operations(&app, config.clone());
    ui_handlers::setup_settings(&app, config.clone());
    ui_handlers::setup_about(&app);

    app.run()?;
    Ok(())
}
