use std::fs;
pub static PROJECTS: &[&str] = &["zig", "rs", "go", "py"];

/// sort one vec basesd on the other in place
/// :parameter
/// * `first`: the order giving vec
/// * `second`: the vec to be sorted
/// :return
/// * `None`
fn sort_based_on_first_vec(first: &[&str], second: &mut Vec<String>) {
    second.sort_by(|a, b| {
        let index_a = first.iter().position(|&x| x == *a).unwrap_or(usize::MAX);
        let index_b = first.iter().position(|&x| x == *b).unwrap_or(usize::MAX);
        index_a.cmp(&index_b)
    });
}
/// Formatting rules per project
/// `split_idx`: which part of the split version info is the info
/// `split_len`: how long the split version info has to be
/// `emoji`: the emoji to use as indicator
/// `compiler`: command to get the version info
/// `version_command`: arg to get the version
pub struct ProjSetting {
    pub split_idx: usize,
    pub split_len: usize,
    pub emoji: String,
    pub compiler: String,
    pub version_command: String,
}

/// get the ProjSetting struct for all projects
///
/// :parameter
/// * `None`
/// :return
/// * the structs for all projects
pub fn get_proj_settings() -> (ProjSetting, ProjSetting, ProjSetting, ProjSetting) {
    let python = ProjSetting {
        split_idx: 1,
        split_len: 2,
        emoji: "🐍".to_string(),
        compiler: "python3".to_string(),
        version_command: "--version".to_string(),
    };
    let zig: ProjSetting = ProjSetting {
        split_idx: 0,
        split_len: 1,
        emoji: "🐊".to_string(),
        compiler: "zig".to_string(),
        version_command: "version".to_string(),
    };
    let rust: ProjSetting = ProjSetting {
        split_idx: 1,
        split_len: 4,
        emoji: "🦀".to_string(),
        compiler: "rustc".to_string(),
        version_command: "--version".to_string(),
    };
    let go: ProjSetting = ProjSetting {
        split_idx: 2,
        split_len: 4,
        emoji: "🐿️".to_string(),
        compiler: "go".to_string(),
        version_command: "version".to_string(),
    };
    (python, zig, rust, go)
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
    if found.len() > 1 {
        sort_based_on_first_vec(&PROJECTS, &mut found);
    }
    found
}
