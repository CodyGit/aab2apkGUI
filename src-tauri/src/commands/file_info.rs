use crate::bundletool::parser::{parse_manifest_output, AppFileInfo};

#[tauri::command]
pub fn read_aab_info(app: tauri::AppHandle, aab_path: String) -> Result<AppFileInfo, String> {
    let settings = crate::config::load(&app)?;
    let jar_path = &settings.bundletool_jar_path;

    if jar_path.is_empty() {
        return Err("请先配置 bundletool 路径".into());
    }

    let mut cmd = crate::bundletool::build_dump_command(jar_path, &aab_path);
    let output = cmd
        .output()
        .map_err(|e| format!("执行 bundletool 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    if !output.status.success() {
        return Err(format!("读取 AAB 信息失败: {}", combined));
    }

    Ok(parse_manifest_output(&combined))
}

#[tauri::command]
pub fn read_apk_info(app: tauri::AppHandle, apk_path: String) -> Result<AppFileInfo, String> {
    let settings = crate::config::load(&app)?;
    let jar_path = &settings.bundletool_jar_path;

    if jar_path.is_empty() {
        return Err("请先配置 bundletool 路径".into());
    }

    let mut cmd = crate::bundletool::build_dump_command(jar_path, &apk_path);
    let output = cmd
        .output()
        .map_err(|e| format!("执行 bundletool 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    if !output.status.success() {
        return Err(format!("读取 APK 信息失败: {}", combined));
    }

    Ok(parse_manifest_output(&combined))
}
