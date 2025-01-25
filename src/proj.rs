use rayon::prelude::*;

use crate::utils::{CHIPMUNK, CRAB, SNAKE, VOLTAGE};
use std::{collections::HashSet, fs};

pub static PROJECTS: &[&str] = &["zig", "rs", "go", "py"];

/// sort one vec basesd on the other in place
/// :parameter
/// * `first`: the order giving vec
/// * `second`: the vec to be sorted
///     :return
/// * `None`
fn sort_based_on_first_vec(first: &[&str], second: &mut [String]) {
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
    pub emoji: char,
    pub compiler: String,
    pub version_command: String,
}

/// get the ProjSetting struct for all projects
///
/// :parameter
/// * `None`
///     :return
/// * the structs for all projects
pub fn get_proj_settings() -> (ProjSetting, ProjSetting, ProjSetting, ProjSetting) {
    let python = ProjSetting {
        split_idx: 1,
        split_len: 2,
        emoji: SNAKE,
        compiler: "python3".to_string(),
        version_command: "--version".to_string(),
    };
    let zig: ProjSetting = ProjSetting {
        split_idx: 0,
        split_len: 1,
        emoji: VOLTAGE,
        compiler: "zig".to_string(),
        version_command: "version".to_string(),
    };
    let rust: ProjSetting = ProjSetting {
        split_idx: 1,
        split_len: 4,
        emoji: CRAB,
        compiler: "rustc".to_string(),
        version_command: "--version".to_string(),
    };
    let go: ProjSetting = ProjSetting {
        split_idx: 2,
        split_len: 4,
        emoji: CHIPMUNK,
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
pub fn is_proj(pwd: &str, file_ending: &[&str]) -> Option<Vec<String>> {
    if let Ok(paths) = fs::read_dir(pwd) {
        let avail_paths = paths
            .filter_map(|x| {
                x.ok().and_then(|e| {
                    e.path()
                        .extension()
                        .and_then(|z| z.to_str().map(String::from))
                })
            })
            .collect::<HashSet<String>>();
        let mut found = file_ending
            .par_iter()
            .filter(|x| avail_paths.contains(**x))
            .map(|x| String::from(*x))
            .collect::<Vec<String>>();
        sort_based_on_first_vec(PROJECTS, &mut found);
        Some(found.to_vec())
    } else {
        None
    }
}
