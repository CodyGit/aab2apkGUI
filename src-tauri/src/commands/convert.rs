use crate::bundletool::{self, BundletoolArgs};
use crate::config::KeystoreConfig;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Serialize, Clone)]
pub struct ConvertProgress {
    pub line: String,
    pub done: bool,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apk_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConvertResult {
    pub success: bool,
    #[serde(rename = "apkPath")]
    pub apk_path: String,
    #[serde(rename = "outputDir")]
    pub output_dir: String,
    #[serde(rename = "apkFiles")]
    pub apk_files: Vec<String>,
}

#[tauri::command]
pub async fn convert_aab(
    app: AppHandle,
    aab_path: String,
    output_dir: String,
    keystore_config: KeystoreConfig,
    device_spec: Option<String>,
) -> Result<ConvertResult, String> {
    let settings = crate::config::load(&app)?;

    let temp_dir = std::env::temp_dir().join(format!("aab2apk_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("创建临时目录失败: {}", e))?;

    let apks_path = temp_dir
        .join("output.apks")
        .to_string_lossy()
        .to_string();

    let (mode, device_spec_path) = if let Some(ref spec) = device_spec {
        if spec.trim().is_empty() {
            (Some("universal".to_string()), None)
        } else if let Ok(val) = serde_json::from_str::<serde_json::Value>(spec) {
            // Check if it's the special universal marker
            if val.get("mode").and_then(|v| v.as_str()) == Some("universal") {
                (Some("universal".to_string()), None)
            } else {
                let spec_file = temp_dir.join("device-spec.json");
                std::fs::write(&spec_file, spec)
                    .map_err(|e| format!("写入设备配置临时文件失败: {}", e))?;
                (None, Some(spec_file.to_string_lossy().to_string()))
            }
        } else {
            // Plain file path
            (None, Some(spec.clone()))
        }
    } else {
        (Some("universal".to_string()), None)
    };

    let args = BundletoolArgs {
        jar_path: settings.bundletool_jar_path.clone(),
        bundle_path: aab_path.clone(),
        output_apks: apks_path.clone(),
        keystore_path: keystore_config.keystore_path.clone(),
        store_password: keystore_config.store_password.clone(),
        key_alias: keystore_config.key_alias.clone(),
        key_password: keystore_config.key_password.clone(),
        device_spec: device_spec_path,
        mode,
    };

    let mut cmd = bundletool::build_command(&args);

    // Log the full command
    let cmd_str = format!(
        "java -jar \"{}\" build-apks --bundle=\"{}\" --output=\"{}\" --ks=\"{}\" --ks-pass=pass:**** --ks-key-alias=\"{}\" --key-pass=pass:****{}{}",
        args.jar_path,
        args.bundle_path,
        args.output_apks,
        args.keystore_path,
        args.key_alias,
        args.device_spec.as_ref().map_or(String::new(), |s| format!(" --device-spec=\"{}\"", s)),
        args.mode.as_ref().map_or(String::new(), |m| format!(" --mode={}", m)),
    );
    emit_progress(&app, format!("[命令] {}", cmd_str), false, false, None, None);
    emit_progress(&app, "开始转换...".into(), false, false, None, None);

    let output = cmd
        .output()
        .map_err(|e| format!("执行 bundletool 失败: {}. 请检查 Java 和 bundletool 配置。", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    for line in stdout.lines() {
        emit_progress(&app, line.to_string(), false, false, None, None);
    }
    for line in stderr.lines() {
        if !line.is_empty() {
            emit_progress(&app, line.to_string(), false, false, None, None);
        }
    }

    if !output.status.success() {
        let err_msg = format!(
            "转换失败 (exit code: {}): {}",
            output.status.code().unwrap_or(-1),
            stderr
        );
        emit_progress(&app, err_msg.clone(), true, false, None, Some(err_msg.clone()));
        return Err(err_msg);
    }

    emit_progress(&app, "正在解压 APK...".into(), false, false, None, None);

    let mut apk_files = bundletool::extract_apk(&apks_path, &output_dir)?;

    // Rename APKs to <base>-<timestamp>.apk
    let base_name = std::path::Path::new(&aab_path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("output".to_string());
    let now = chrono::Local::now();
    let ts = now.format("%m%d%H%M").to_string();
    let mut renamed_files = Vec::new();
    for (i, apk) in apk_files.iter().enumerate() {
        let new_name = if apk_files.len() == 1 {
            format!("{}-{}.apk", base_name, ts)
        } else {
            format!("{}-{}-{}.apk", base_name, ts, i + 1)
        };
        let new_path = std::path::Path::new(&output_dir).join(&new_name);
        std::fs::rename(apk, &new_path)
            .map_err(|e| format!("重命名 APK 失败: {}", e))?;
        renamed_files.push(new_path);
    }
    apk_files = renamed_files;

    // Clean up temp
    let _ = std::fs::remove_dir_all(&temp_dir);

    let apk_path = apk_files
        .first()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    emit_progress(
        &app,
        format!("转换完成！共生成 {} 个 APK 文件", apk_files.len()),
        true,
        true,
        Some(apk_path.clone()),
        None,
    );

    Ok(ConvertResult {
        success: true,
        apk_path,
        output_dir,
        apk_files: apk_files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect(),
    })
}

fn emit_progress(
    app: &AppHandle,
    line: String,
    done: bool,
    success: bool,
    apk_path: Option<String>,
    error: Option<String>,
) {
    let _ = app.emit("convert-progress", ConvertProgress {
        line,
        done,
        success,
        apk_path,
        error,
    });
}
