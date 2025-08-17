use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;

pub fn collect_files_from_paths(paths: &[String], recursive: bool) -> Vec<PathBuf> {
    let mut all_files = vec![];
    
    for path_str in paths {
        let path = PathBuf::from(path_str);
        
        if path.is_file() {
            all_files.push(path);
        } else if path.is_dir() {
            if recursive {
                // 递归扫描目录
                for entry in WalkDir::new(&path).into_iter().flatten() {
                    if entry.path().is_file() {
                        all_files.push(entry.path().to_path_buf());
                    }
                }
            } else {
                // 只扫描直接子文件
                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        if entry.path().is_file() {
                            all_files.push(entry.path());
                        }
                    }
                }
            }
        }
    }
    
    all_files
}

pub fn convert_file_extension_case(path: &Path, to_upper: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .ok_or("文件没有扩展名")?;
    
    let new_ext = if to_upper {
        ext.to_uppercase()
    } else {
        ext.to_lowercase()
    };
    
    if new_ext == ext {
        return Err("扩展名无需更改".into());
    }
    
    let new_path = path.with_extension(new_ext);
    fs::rename(path, &new_path)?;
    Ok(new_path)
}

pub fn select_files() -> Option<Vec<PathBuf>> {
    rfd::FileDialog::new()
        .add_filter("所有文件", &["*"])
        .set_title("选择要转换的文件")
        .pick_files()
}

#[allow(dead_code)]
pub fn select_folders() -> Option<Vec<PathBuf>> {
    if let Some(folder) = rfd::FileDialog::new()
        .set_title("选择要转换的文件夹")
        .pick_folder() {
        Some(vec![folder])
    } else {
        None
    }
}
