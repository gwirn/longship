use std::fs;
pub static PROJECTS: &[&str] = &["rs", "go", "py"];
/// Formatting rules per project
/// `split_idx`: which part of the split version info is the info
/// `split_len`: how long the split version info has to be
/// `emoji`: the emoji to use as indicator
/// `emoji_space_add`: how much to adjust the padding
/// `compiler`: command to get the version info
/// `version_command`: arg to get the version
pub struct ProjSetting {
    pub split_idx: usize,
    pub split_len: usize,
    pub emoji: String,
    pub emoji_space_add: usize,
    pub compiler: String,
    pub version_command: String,
}

/// get the ProjSetting struct for all projects
///
/// :parameter
/// * `None`
/// :return
/// * the structs for all projects
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

/// which project(s) we are dealing with and for which info should be displayed
///
/// :parameter
/// * `pwd`: current directory
/// * `file_ending`: for which file extenions should be searched
///
/// :return
/// * `found`: all unique file extensions
pub fn is_proj(pwd: &str, file_ending: &[&str]) -> Vec<String> {
    let mut proj_found = 0;
    let proj_to_search = file_ending.len();
    let mut found: Vec<String> = Vec::with_capacity(proj_to_search);
    if let Ok(files) = fs::read_dir(pwd) {
        for f in files.into_iter() {
            if proj_found == proj_to_search {
                break;
            }
            if let Ok(file_name) = f {
                if let Some(file_ext) = file_name.path().extension().and_then(|x| x.to_str()) {
                    // is it a extension we are looking for
                    let yes_proj = file_ending.contains(&file_ext);
                    let fext = file_ext.to_string();
                    // to avoid duplicates
                    if yes_proj && !found.contains(&fext) {
                        found.push(fext);
                        proj_found += 1;
                    }
                }
            }
        }
    }
    found
}
