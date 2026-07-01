mod commands;
mod bundletool;
mod config;

use config::AppSettings;
use std::process::Command;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;

            // Create default config if not exists
            let config_path = app_dir.join("config.json");
            if !config_path.exists() {
                let defaults = AppSettings::default();
                config::save_to_path(&config_path, &defaults)?;
            }

            // Generate default device configs
            let dc_dir = app_dir.join("device-configs");
            std::fs::create_dir_all(&dc_dir).ok();
            let defaults: Vec<(&str, &str)> = vec![
                ("universal", r#"{"mode":"universal"}"#),
                ("Pixel 8 series", r#"{"supportedAbis":["arm64-v8a"],"screenDensity":480,"sdkVersion":34,"supportedLocales":["zh-CN","en-US"]}"#),
                ("主流中端机", r#"{"supportedAbis":["arm64-v8a","armeabi-v7a"],"screenDensity":320,"sdkVersion":28}"#),
                ("纯64位设备", r#"{"supportedAbis":["arm64-v8a"],"sdkVersion":31}"#),
            ];
            for (name, config) in &defaults {
                let p = dc_dir.join(format!("{}.json", name));
                if !p.exists() {
                    let _ = std::fs::write(&p, config);
                }
            }

            // Generate debug.keystore in keystores dir if not present
            let ks_dir = app_dir.join("keystores");
            std::fs::create_dir_all(&ks_dir).ok();
            let ks_path = ks_dir.join("debug.keystore");
            if !ks_path.exists() {
                let result = Command::new("keytool")
                    .arg("-genkey")
                    .arg("-v")
                    .arg("-keystore")
                    .arg(&ks_path)
                    .arg("-alias")
                    .arg("androiddebugkey")
                    .arg("-keyalg")
                    .arg("RSA")
                    .arg("-keysize")
                    .arg("2048")
                    .arg("-validity")
                    .arg("10000")
                    .arg("-storepass")
                    .arg("android")
                    .arg("-keypass")
                    .arg("android")
                    .arg("-dname")
                    .arg("CN=Android Debug,O=Android,C=US")
                    .output();
                if result.is_ok() {
                    let conf = serde_json::json!({
                        "keystorePath": ks_path.to_string_lossy(),
                        "storePassword": "android",
                        "keyAlias": "androiddebugkey",
                        "keyPassword": "android"
                    });
                    let ks_conf_path = ks_dir.join("debug.keystore.json");
                    let _ = std::fs::write(&ks_conf_path, conf.to_string());
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::env_check::check_java,
            commands::env_check::check_bundletool,
            commands::env_check::env_check,
            commands::settings::load_settings,
            commands::settings::save_settings,
            commands::device_config::get_device_configs_dir,
            commands::device_config::list_device_configs,
            commands::device_config::add_device_config,
            commands::settings::reveal_in_explorer,
            commands::keystore::get_keystores_dir,
            commands::keystore::list_keystores,
            commands::keystore::list_keystore_info,
            commands::keystore::validate_keystore,
            commands::keystore::parse_keystore_config,
            commands::keystore::add_keystore,
            commands::convert::convert_aab,
            commands::file_info::read_aab_info,
            commands::file_info::read_apk_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
