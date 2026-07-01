use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct AppFileInfo {
    #[serde(rename = "packageName")]
    pub package_name: String,
    #[serde(rename = "versionName")]
    pub version_name: String,
    #[serde(rename = "versionCode")]
    pub version_code: String,
    #[serde(rename = "minSdkVersion")]
    pub min_sdk_version: String,
    #[serde(rename = "targetSdkVersion")]
    pub target_sdk_version: String,
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
}

pub fn parse_manifest_output(output: &str) -> AppFileInfo {
    let mut info = AppFileInfo::default();

    for line in output.lines() {
        let line = line.trim();
        if line.starts_with("package:") {
            info.package_name = line
                .strip_prefix("package:")
                .unwrap_or("")
                .trim()
                .trim_matches('\'')
                .to_string();
        } else if line.starts_with("versionName:") {
            info.version_name = line
                .strip_prefix("versionName:")
                .unwrap_or("")
                .trim()
                .trim_matches('\'')
                .to_string();
        } else if line.starts_with("versionCode:") {
            info.version_code = line
                .strip_prefix("versionCode:")
                .unwrap_or("")
                .trim()
                .trim_matches('\'')
                .to_string();
        } else if line.starts_with("sdkVersion:") || line.starts_with("minSdkVersion:") {
            info.min_sdk_version = line
                .strip_prefix("sdkVersion:")
                .or_else(|| line.strip_prefix("minSdkVersion:"))
                .unwrap_or("")
                .trim()
                .trim_matches('\'')
                .to_string();
        } else if line.starts_with("targetSdkVersion:") {
            info.target_sdk_version = line
                .strip_prefix("targetSdkVersion:")
                .unwrap_or("")
                .trim()
                .trim_matches('\'')
                .to_string();
        } else if line.starts_with("uses-permission:") {
            let perm = line
                .strip_prefix("uses-permission:")
                .unwrap_or("")
                .trim()
                .trim_matches('\'')
                .to_string();
            if !perm.is_empty() {
                info.permissions.push(perm);
            }
        }
    }

    info
}
