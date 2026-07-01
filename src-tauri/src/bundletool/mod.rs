use std::path::PathBuf;
use std::process::Command;

pub mod parser;

pub struct BundletoolArgs {
    pub jar_path: String,
    pub bundle_path: String,
    pub output_apks: String,
    pub keystore_path: String,
    pub store_password: String,
    pub key_alias: String,
    pub key_password: String,
    pub device_spec: Option<String>,
    pub mode: Option<String>,
}

pub fn build_command(args: &BundletoolArgs) -> Command {
    let mut cmd = Command::new("java");
    cmd.arg("-jar")
        .arg(&args.jar_path)
        .arg("build-apks")
        .arg("--bundle")
        .arg(&args.bundle_path)
        .arg("--output")
        .arg(&args.output_apks)
        .arg("--ks")
        .arg(&args.keystore_path)
        .arg("--ks-pass")
        .arg(format!("pass:{}", args.store_password))
        .arg("--ks-key-alias")
        .arg(&args.key_alias)
        .arg("--key-pass")
        .arg(format!("pass:{}", args.key_password));

    if let Some(ref spec) = args.device_spec {
        cmd.arg("--device-spec").arg(spec);
    }

    if let Some(ref mode) = args.mode {
        cmd.arg("--mode").arg(mode);
    }

    cmd
}

pub fn build_dump_command(jar_path: &str, file_path: &str) -> Command {
    let mut cmd = Command::new("java");
    cmd.arg("-jar")
        .arg(jar_path)
        .arg("dump")
        .arg("manifest")
        .arg("--bundle")
        .arg(file_path);
    cmd
}

pub fn extract_apk(apks_path: &str, output_dir: &str) -> Result<Vec<PathBuf>, String> {
    let file = std::fs::File::open(apks_path)
        .map_err(|e| format!("无法打开 .apks 文件: {}", e))?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("无法读取 .apks 文件: {}", e))?;

    let mut extracted = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("读取压缩包条目失败: {}", e))?;
        let name = entry.name().to_string();

        // Extract universal.apk or standalone APKs
        if name.ends_with(".apk") && (name.contains("universal") || name.contains("standalones"))
        {
            let out_path = PathBuf::from(output_dir).join(
                std::path::Path::new(&name)
                    .file_name()
                    .unwrap_or_default(),
            );
            let mut out_file = std::fs::File::create(&out_path)
                .map_err(|e| format!("无法创建输出文件: {}", e))?;
            std::io::copy(&mut entry, &mut out_file)
                .map_err(|e| format!("解压失败: {}", e))?;
            extracted.push(out_path);
        }
    }

    if extracted.is_empty() {
        // Fallback: extract all .apk files under splits/
        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| format!("读取压缩包条目失败: {}", e))?;
            let name = entry.name().to_string();
            if name.ends_with(".apk") {
                let out_path = PathBuf::from(output_dir).join(
                    std::path::Path::new(&name)
                        .file_name()
                        .unwrap_or_default(),
                );
                let mut out_file = std::fs::File::create(&out_path)
                    .map_err(|e| format!("无法创建输出文件: {}", e))?;
                std::io::copy(&mut entry, &mut out_file)
                    .map_err(|e| format!("解压失败: {}", e))?;
                extracted.push(out_path);
            }
        }
    }

    Ok(extracted)
}
