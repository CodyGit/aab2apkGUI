use serde::Serialize;
use std::process::Command;

use crate::config::KeystoreConfig;

#[derive(Debug, Serialize)]
pub struct KeystoreAliasInfo {
    pub name: String,
    pub expires: String,
}

#[derive(Debug, Serialize)]
pub struct KeystoreInfo {
    pub aliases: Vec<KeystoreAliasInfo>,
}

#[derive(Debug, Serialize)]
pub struct KeystoreValidateResult {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct KeystoreEntry {
    pub name: String,
    pub config: KeystoreConfig,
}

#[tauri::command]
pub fn get_keystores_dir(app: tauri::AppHandle) -> Result<String, String> {
    let dir = crate::config::keystores_dir(&app);
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
pub fn list_keystores(app: tauri::AppHandle) -> Result<Vec<KeystoreEntry>, String> {
    let keystore_dir = crate::config::keystores_dir(&app);
    std::fs::create_dir_all(&keystore_dir).map_err(|e| format!("创建证书目录失败: {}", e))?;

    let mut entries = Vec::new();
    if let Ok(dir) = std::fs::read_dir(&keystore_dir) {
        for entry in dir.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(config) = serde_json::from_str::<KeystoreConfig>(&content) {
                        let name = path
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_default();
                        entries.push(KeystoreEntry { name, config });
                    }
                }
            }
        }
    }

    Ok(entries)
}

#[tauri::command]
pub fn list_keystore_info(
    keystore_path: String,
    store_password: String,
) -> Result<KeystoreInfo, String> {
    let output = Command::new("keytool")
        .arg("-list")
        .arg("-v")
        .arg("-keystore")
        .arg(&keystore_path)
        .arg("-storepass")
        .arg(&store_password)
        .output()
        .map_err(|e| format!("执行 keytool 失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("读取证书信息失败: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut aliases = Vec::new();
    let mut current_alias = String::new();

    for line in stdout.lines() {
        if line.starts_with("Alias name:") {
            current_alias = line
                .strip_prefix("Alias name:")
                .unwrap_or("")
                .trim()
                .to_string();
        }
        if line.contains("Valid from:") && !current_alias.is_empty() {
            if let Some(until_idx) = line.find("until:") {
                let expires = line[until_idx + 6..].trim();
                let expires = expires.trim_start_matches('[').trim();
                aliases.push(KeystoreAliasInfo {
                    name: current_alias.clone(),
                    expires: expires.to_string(),
                });
                current_alias.clear();
            }
        }
    }

    Ok(KeystoreInfo { aliases })
}

#[tauri::command]
pub fn validate_keystore(
    keystore_path: String,
    store_password: String,
    key_alias: String,
    key_password: String,
) -> Result<KeystoreValidateResult, String> {
    let output = Command::new("keytool")
        .arg("-list")
        .arg("-keystore")
        .arg(&keystore_path)
        .arg("-storepass")
        .arg(&store_password)
        .arg("-alias")
        .arg(&key_alias)
        .arg("-keypass")
        .arg(&key_password)
        .output()
        .map_err(|e| format!("执行 keytool 失败: {}", e))?;

    if output.status.success() {
        Ok(KeystoreValidateResult {
            valid: true,
            error: None,
        })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let error_msg = if stderr.contains("password") {
            "密码错误".to_string()
        } else if stderr.contains("does not exist") {
            "别名不存在".to_string()
        } else {
            stderr.trim().to_string()
        };
        Ok(KeystoreValidateResult {
            valid: false,
            error: Some(error_msg),
        })
    }
}

#[tauri::command]
pub fn parse_keystore_config(config_path: String) -> Result<KeystoreConfig, String> {
    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("无法读取 keystore 配置文件: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("解析 keystore 配置文件失败: {}", e))
}

#[tauri::command]
pub fn add_keystore(
    app: tauri::AppHandle,
    source_path: String,
    name: String,
    store_password: String,
    key_alias: String,
    key_password: String,
) -> Result<(), String> {
    let keystore_dir = crate::config::keystores_dir(&app);
    std::fs::create_dir_all(&keystore_dir).map_err(|e| format!("创建证书目录失败: {}", e))?;

    let dest_ks = keystore_dir.join(format!("{}.keystore", name));
    std::fs::copy(&source_path, &dest_ks)
        .map_err(|e| format!("复制证书文件失败: {}", e))?;

    let config = KeystoreConfig {
        keystore_path: dest_ks.to_string_lossy().to_string(),
        store_password,
        key_alias,
        key_password,
    };

    let conf_path = keystore_dir.join(format!("{}.keystore.json", name));
    let content =
        serde_json::to_string_pretty(&config).map_err(|e| format!("序列化证书配置失败: {}", e))?;
    std::fs::write(&conf_path, content).map_err(|e| format!("保存证书配置失败: {}", e))
}
