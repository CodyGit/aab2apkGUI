use crate::config::AppSettings;
use std::process::Command;

#[tauri::command]
pub fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    crate::config::load(&app)
}

#[tauri::command]
pub fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    crate::config::save(&app, &settings)
}

#[tauri::command]
pub fn reveal_in_explorer(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    let target = if p.is_dir() {
        path.clone()
    } else if let Some(parent) = p.parent() {
        parent.to_string_lossy().to_string()
    } else {
        path.clone()
    };

    #[cfg(target_os = "windows")]
    {
        let name: String = if p.is_dir() {
            ".".into()
        } else {
            p.file_name().unwrap_or_default().to_string_lossy().into_owned()
        };
        let sel_path = std::path::Path::new(&target).join(&name);
        Command::new("explorer")
            .arg("/select,")
            .arg(sel_path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-R")
            .arg(&target)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&target)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    Ok(())
}
