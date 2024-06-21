use std::fs;
use std::process::Command;
pub struct ProjSetting {
    pub split_idx: usize,
    pub split_len: usize,
    pub emoji: String,
    pub emoji_space_add: usize,
    pub compiler: String,
    pub version_command: String,
}

pub fn get_proj_settings() -> (ProjSetting, ProjSetting, ProjSetting) {
    let python = ProjSetting {
        split_idx: 1,
        split_len: 2,
        emoji: "🐍".to_string(),
        emoji_space_add: 0,
        compiler: "python".to_string(),
        version_command: "--version".to_string(),
    };
    let rust: ProjSetting = ProjSetting {
        split_idx: 1,
        split_len: 4,
        emoji: "🦀".to_string(),
        emoji_space_add: 0,
        compiler: "rustc".to_string(),
        version_command: "--version".to_string(),
    };
    let go: ProjSetting = ProjSetting {
        split_idx: 2,
        split_len: 4,
        emoji: "🐿️".to_string(),
        emoji_space_add: 4,
        compiler: "go".to_string(),
        version_command: "version".to_string(),
    };
    (python, rust, go)
}
pub fn is_proj(pwd: &str, file_ending: &[&str]) -> Vec<String> {
    let mut found: Vec<String> = Vec::with_capacity(3);
    if let Ok(files) = fs::read_dir(pwd) {
        for f in files.into_iter() {
            if let Ok(file_name) = f {
                if let Some(file_ext) = file_name.path().extension().and_then(|x| x.to_str()) {
                    let yes_proj = file_ending.contains(&file_ext);
                    let fext = file_ext.to_string();
                    if yes_proj && !found.contains(&fext) {
                        found.push(fext)
                    }
                }
            }
        }
    }
    found
}
pub fn proj_version(com: &str, arg: &str) -> String {
    match Command::new(com).arg(arg).output() {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(o) => o,
            Err(_) => "".to_string(),
        },
        Err(_) => "".to_string(),
    }
}
