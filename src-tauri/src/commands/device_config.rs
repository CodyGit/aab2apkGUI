use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct DeviceConfigEntry {
    pub name: String,
    pub config: String,
}

#[tauri::command]
pub fn get_device_configs_dir(app: tauri::AppHandle) -> Result<String, String> {
    let dir = crate::config::device_configs_dir(&app);
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
pub fn list_device_configs(app: tauri::AppHandle) -> Result<Vec<DeviceConfigEntry>, String> {
    let dir = crate::config::device_configs_dir(&app);
    std::fs::create_dir_all(&dir).map_err(|e| format!("创建设备配置目录失败: {}", e))?;

    let mut entries = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let name = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();
                    entries.push(DeviceConfigEntry { name, config: content });
                }
            }
        }
    }

    Ok(entries)
}

#[tauri::command]
pub fn add_device_config(
    app: tauri::AppHandle,
    name: String,
    config: String,
) -> Result<(), String> {
    // Validate JSON
    serde_json::from_str::<serde_json::Value>(&config)
        .map_err(|e| format!("JSON 格式错误: {}", e))?;

    let dir = crate::config::device_configs_dir(&app);
    std::fs::create_dir_all(&dir).map_err(|e| format!("创建设备配置目录失败: {}", e))?;

    let path = dir.join(format!("{}.json", name));
    std::fs::write(&path, &config).map_err(|e| format!("保存设备配置失败: {}", e))
}

#[tauri::command]
pub fn delete_device_config(app: tauri::AppHandle, name: String) -> Result<(), String> {
    let dir = crate::config::device_configs_dir(&app);
    let path = dir.join(format!("{}.json", name));
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("删除设备配置失败: {}", e))
    } else {
        Err("配置文件不存在".into())
    }
}
