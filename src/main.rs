use file_search::*;
use format::*;
use proj::*;
use std::env;
use std::usize;
use terminal_size::{terminal_size, Height, Width};
use utils::*;

mod file_search;
mod format;
mod proj;
mod utils;

static PROJECTS: &[&str] = &["rs", "go", "py"];

fn main() {
    let orange = 208;
    let grey = 245;
    let blue = 24;
    let green = 65;
    let red = 9;
    let turquoise = 45;
    let carb_orange = 166;
    let (python, rust, go) = get_proj_settings();
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
                &red
            } else {
                &green
            }
        }),
        path_string,
        color_and_esc(&git_info, &shell, &grey),
    );

    let mut py = "".to_string();
    if !conda.is_empty() {
        py = get_filename(conda);
    } else if !venv.is_empty() {
        py = get_filename(venv);
    }

    let mut proj_len = py.len();
    let mut proj_string: String = "".to_string();

    for ext in is_proj(&pwd, PROJECTS) {
        match &*ext {
            "rs" => proj_format(
                &rust,
                &mut proj_len,
                &mut proj_string,
                &mut emoji_space,
                &shell,
                &carb_orange,
            ),
            "go" => proj_format(
                &go,
                &mut proj_len,
                &mut proj_string,
                &mut emoji_space,
                &shell,
                &turquoise,
            ),
            "py" => proj_format(
                &python,
                &mut proj_len,
                &mut proj_string,
                &mut emoji_space,
                &shell,
                &blue,
            ),
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
        color_and_esc(&py, &shell, &blue),
        color_and_esc("»", &shell, &orange)
    );
    println!("\r{} ", path);
}
