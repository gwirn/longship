use file_search::*;
use format::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;
use std::usize;
use terminal_size::{terminal_size, Height, Width};
use utils::*;

mod file_search;
mod format;
mod utils;

fn is_proj(pwd: &str, file_ending: &str) -> bool {
    let mut yes_proj = false;
    if let Ok(files) = fs::read_dir(pwd) {
        for f in files.into_iter() {
            if let Ok(file_name) = f {
                if let Some(file_ext) = file_name.path().extension().and_then(|x| x.to_str()) {
                    yes_proj = file_ext == file_ending;
                    if yes_proj {
                        break;
                    }
                }
            }
        }
    }
    yes_proj
}
fn proj_version(com: &str, arg: &str) -> String {
    match Command::new(com).arg(arg).output() {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(o) => o,
            Err(_) => "".to_string(),
        },
        Err(_) => "".to_string(),
    }
}
fn main() {
    // let file_extensions = HashMap::from([("py", "python"), ("rs", "rust"), ("go", "go")]);
    let projects = HashMap::from([
        ("python", ("py", ["python", "--version"])),
        ("rust", ("rs", ["rustc", "--version"])),
        ("go", ("go", ["go", "version"])),
    ]);
    let orange = 208;
    let grey = 245;
    let blue = 24;
    let green = 65;
    let red = 9;
    let turquoise = 45;
    let carb_orange = 166;
    env::set_var("CLICOLOR_FORCE", "1");

    let shell = get_shell();
    let home = get_env_var("HOME");
    let conda = get_env_var("CONDA_PREFIX");
    let venv = get_env_var("VIRTUAL_ENV");

    let pwd = match env::current_dir() {
        Ok(path) => path.to_str().unwrap_or_default().to_string(),
        Err(_) => String::new(),
    };

    let path_string = match pwd.strip_prefix(&home) {
        Some(p) => format!("~{}", p),
        None => "".to_string(),
    };
    let size = terminal_size();
    let w: usize = match size {
        Some((Width(ww), Height(_))) => ww.into(),
        None => 0,
    };

    let mut emoji_space = 0;
    let mut git_info = git_branch();
    if !git_info.is_empty() {
        git_info = format!("🌿{}", git_info);
        emoji_space += 2;
    }
    let (user, mut ssh_string) = get_ssh();
    let mut is_root = user.to_lowercase() == "root";
    if !ssh_string.is_empty() {
        ssh_string = format!("{} [{}]", "🛰️", ssh_string);
        emoji_space += 6;
    } else if !user.is_empty() {
        ssh_string = "[ROOT]".to_string();
        is_root = true;
    }

    let mut path = format!(
        "{}{} {}",
        color_and_esc(&ssh_string, &shell, {
            if is_root {
                red
            } else {
                green
            }
        }),
        path_string,
        color_and_esc(&git_info, &shell, grey),
    );

    let mut py = "".to_string();
    if !conda.is_empty() {
        py = get_filename(conda);
    } else if !venv.is_empty() {
        py = get_filename(venv);
    }

    let mut proj_len = py.len();
    let mut proj_string: String = "".to_string();
    for (p, c) in projects.into_iter() {
        match p {
            "python" => {
                let (ending, com) = c;
                if is_proj(&pwd, ending) {
                    let proj_raw = proj_version(com[0], com[1]);
                    let proj_split: Vec<_> = proj_raw.trim().split(' ').collect();
                    if proj_split.len() == 2 {
                        let pre_ps = format!("🐍{}", proj_split[1]);
                        proj_len += pre_ps.len();
                        proj_string =
                            format!("{proj_string} {} ", color_and_esc(&pre_ps, &shell, blue));
                    }
                }
            }
            "rust" => {
                let (ending, com) = c;
                if is_proj(&pwd, ending) {
                    let proj_raw = proj_version(com[0], com[1]);
                    let proj_split: Vec<_> = proj_raw.trim().split(' ').collect();
                    if proj_split.len() == 4 {
                        let pre_ps = format!("🦀{}", proj_split[1]);
                        proj_len += pre_ps.len();
                        proj_string = format!(
                            "{proj_string} {} ",
                            color_and_esc(&pre_ps, &shell, carb_orange)
                        );
                    }
                }
            }
            "go" => {
                let (ending, com) = c;
                if is_proj(&pwd, ending) {
                    let proj_raw = proj_version(com[0], com[1]);
                    let proj_split: Vec<_> = proj_raw.trim().split(' ').collect();
                    if proj_split.len() == 4 {
                        let pre_ps = format!("🐿️{}", proj_split[2]);
                        proj_len += pre_ps.len();
                        proj_string = format!(
                            "{proj_string} {} ",
                            color_and_esc(&pre_ps, &shell, turquoise)
                        );
                        emoji_space += 4
                    }
                }
            }
            _ => {}
        }
    }

    let path_len = ssh_string.len() + path_string.len() + 1 + git_info.len() - emoji_space;
    let mut padding_size = 1;
    if w >= (path_len + proj_len) {
        padding_size = w - path_len - proj_len;
    };
    let padding = vec![" "; padding_size].join("");
    path = format!(
        "{}{}{}{}\n{}",
        path,
        padding,
        proj_string,
        color_and_esc(&py, &shell, blue),
        color_and_esc("»", &shell, orange)
    );
    println!("\r{} ", path);
}
