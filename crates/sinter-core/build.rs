use std::env;
use std::fs::{self, Permissions};
use std::path::Path;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn main() {
    println!("cargo:rerun-if-changed=templates/i18n.json");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let i18n_path = Path::new(&manifest_dir).join("templates/i18n.json");
    let i18n_content = fs::read_to_string(&i18n_path).expect("Failed to read i18n.json");

    let translations: serde_json::Value = serde_json::from_str(&i18n_content).expect("Failed to parse i18n.json");

    let lang = if cfg!(feature = "lang-zh") { "zh" } else { "en" };
    let lang_translations = translations.get(lang).and_then(|v| v.as_object()).expect(&format!("Language '{}' not found", lang));

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("i18n.rs");

    let mut code = String::new();
    code.push_str("// Auto-generated i18n code\n");
    code.push_str("pub fn t(key: &str) -> &'static str {\n match key {\n");
    for (key, value) in lang_translations {
        let escaped = value.as_str().unwrap().replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
        code.push_str(&format!(" \"{}\" => \"{}\",\n", key, escaped));
    }
    code.push_str(" _ => \"[Translation key not found]\",\n }\n}\n\n");
    code.push_str("pub fn tf(key: &str, args: &[&str]) -> String {\n let template = t(key);\n let mut result = template.to_string();\n for arg in args {\n if let Some(pos) = result.find(\"{}\") {\n let (left, right) = result.split_at(pos);\n result = format!(\"{}{}{}\", left, arg, &right[2..]);\n } }\n result\n}\n");

    fs::write(&out_path, code).expect("Failed to write i18n.rs");

    // 下载coursier（简化平台检测）
    download_coursier();
}

fn download_coursier() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bin_dir = Path::new(&manifest_dir).join("bin");
    fs::create_dir_all(&bin_dir).ok();

    let exe_name = if cfg!(target_os = "windows") { "coursier.exe" } else { "coursier" };
    let coursier_path = bin_dir.join(exe_name);

    if coursier_path.exists() && Command::new(&coursier_path).arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        println!("cargo:warning=coursier already exists");
        return;
    }

    let platform = match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => "x86_64-pc-linux",
        ("linux", "aarch64") => "aarch64-pc-linux",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        _ => return,
    };

    let url = format!("https://github.com/coursier/coursier/releases/latest/download/cs-{}.gz", platform);
    Command::new("sh").args(&["-c", &format!("curl -fL {} | gzip -d > {}", url, coursier_path.display())]).status().ok();

    #[cfg(unix)]
    {
        let perms = Permissions::from_mode(0o755);
        fs::set_permissions(&coursier_path, perms).ok();
    }

    // 拷贝到target/bin
    let binding = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&binding).parent().unwrap().parent().unwrap().parent().unwrap();
    let target_bin_dir = out_dir.join("bin");
    fs::create_dir_all(&target_bin_dir).ok();
    let target_path = target_bin_dir.join(exe_name);
    fs::copy(&coursier_path, &target_path).ok();

    #[cfg(unix)]
    {
        let perms = Permissions::from_mode(0o755);
        fs::set_permissions(&target_path, perms).ok();
    }
}