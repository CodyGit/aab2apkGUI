use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoreConfig {
    #[serde(rename = "keystorePath")]
    pub keystore_path: String,
    #[serde(rename = "storePassword")]
    pub store_password: String,
    #[serde(rename = "keyAlias")]
    pub key_alias: String,
    #[serde(rename = "keyPassword")]
    pub key_password: String,
}

impl Default for KeystoreConfig {
    fn default() -> Self {
        Self {
            keystore_path: String::new(),
            store_password: String::new(),
            key_alias: String::new(),
            key_password: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(rename = "bundletoolJarPath")]
    pub bundletool_jar_path: String,
    #[serde(rename = "defaultOutputDir")]
    pub default_output_dir: String,
    #[serde(rename = "lastKeystoreConfig")]
    pub last_keystore_config: KeystoreConfig,
    #[serde(rename = "lastDeviceTemplate")]
    pub last_device_template: String,
    #[serde(rename = "language")]
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            bundletool_jar_path: String::new(),
            default_output_dir: String::new(),
            last_keystore_config: KeystoreConfig::default(),
            last_device_template: "universal".to_string(),
            language: "zh-CN".to_string(),
        }
    }
}

pub fn config_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join("config.json")
}

pub fn keystores_dir(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join("keystores")
}

pub fn device_configs_dir(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join("device-configs")
}

pub fn load(app: &tauri::AppHandle) -> Result<AppSettings, String> {
    let path = config_path(app);
    load_from_path(&path)
}

pub fn load_from_path(path: &std::path::PathBuf) -> Result<AppSettings, String> {
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("读取配置文件失败: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("解析配置文件失败: {}", e))
}

pub fn save(app: &tauri::AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = config_path(app);
    save_to_path(&path, settings)
}

pub fn save_to_path(path: &std::path::PathBuf, settings: &AppSettings) -> Result<(), String> {
    let content =
        serde_json::to_string_pretty(settings).map_err(|e| format!("序列化配置失败: {}", e))?;
    std::fs::write(path, content).map_err(|e| format!("保存配置文件失败: {}", e))
}
