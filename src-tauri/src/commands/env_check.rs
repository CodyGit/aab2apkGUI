use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize)]
pub struct EnvStatus {
    #[serde(rename = "javaOk")]
    pub java_ok: bool,
    #[serde(rename = "javaVersion")]
    pub java_version: String,
    #[serde(rename = "bundletoolOk")]
    pub bundletool_ok: bool,
    #[serde(rename = "bundletoolPath")]
    pub bundletool_path: String,
}

#[tauri::command]
pub fn check_java() -> Result<bool, String> {
    let output = Command::new("java").arg("--version").output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let combined = format!("{}{}", stdout, stderr);
            // "java version \"11.0.x\"" 或 "openjdk version \"17.0.x\""
            if let Some(ver) = parse_java_version(&combined) {
                Ok(ver >= 11)
            } else {
                Err("无法解析 Java 版本".into())
            }
        }
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub fn check_bundletool(app: tauri::AppHandle) -> Result<bool, String> {
    let settings = crate::config::load(&app)?;
    if settings.bundletool_jar_path.is_empty() {
        return Ok(false);
    }
    let path = std::path::Path::new(&settings.bundletool_jar_path);
    Ok(path.exists())
}

#[tauri::command]
pub fn env_check(app: tauri::AppHandle) -> Result<EnvStatus, String> {
    let java_version = get_java_version();
    let java_ok = java_version.contains(" ") // "11.x" or "17.x" etc
        || !java_version.is_empty();
    let settings = crate::config::load(&app).unwrap_or_default();
    let bundletool_path = settings.bundletool_jar_path.clone();
    let bundletool_ok = if bundletool_path.is_empty() {
        false
    } else {
        std::path::Path::new(&bundletool_path).exists()
    };

    Ok(EnvStatus {
        java_ok,
        java_version,
        bundletool_ok,
        bundletool_path,
    })
}

fn get_java_version() -> String {
    let output = Command::new("java").arg("--version").output();
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let combined = format!("{}{}", stdout, stderr);
            if let Some(ver) = parse_java_version(&combined) {
                format!("Java {}", ver)
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

fn parse_java_version(output: &str) -> Option<u32> {
    // Handle formats:
    //   openjdk 17.0.17 2025-10-21     (modern OpenJDK)
    //   java version "11.0.2"          (old Oracle)
    //   openjdk version "17.0.17"      (old OpenJDK)
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Try to find "version" keyword
        if let Some(idx) = trimmed.find("version") {
            let rest = &trimmed[idx + 7..].trim().trim_matches('"');
            // e.g. "17.0.2" or "11"
            if let Some(v) = extract_major_version(rest) {
                return Some(v);
            }
        }

        // Modern OpenJDK: "openjdk 17.0.17 2025-10-21"
        // The second token is the version number
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 && parts[0].starts_with("openjdk") {
            if let Some(v) = extract_major_version(parts[1]) {
                return Some(v);
            }
        }

        // Fallback: first number pattern in the line
        if let Some(v) = extract_major_version(trimmed) {
            return Some(v);
        }
    }
    None
}

fn extract_major_version(s: &str) -> Option<u32> {
    let s = s.trim().trim_matches('"').trim_matches('\'');
    // e.g. "17.0.17" -> 17, or "1.8.0" -> 8 (Java 8 special case)
    let first_num: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if let Ok(v) = first_num.parse::<u32>() {
        if v == 1 {
            // Could be Java 1.8 format -> return 8
            let after_one = &s[first_num.len()..].trim().trim_matches('.').trim_matches('"');
            let second_num: String = after_one.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(v2) = second_num.parse::<u32>() {
                return Some(v2);
            }
        }
        return Some(v);
    }
    None
}
