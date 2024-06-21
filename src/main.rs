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

fn main() {
    // get the setting struct for all supported projects
    let (python, rust, go) = get_proj_settings();
    env::set_var("CLICOLOR_FORCE", "1");

    // get which shell is used, home path and python env
    let shell = get_shell();
    let home = get_env_var("HOME");
    let conda = get_env_var("CONDA_PREFIX");
    let venv = get_env_var("VIRTUAL_ENV");

    // path current working dir
    let pwd = match env::current_dir() {
        Ok(path) => path.to_str().unwrap_or_default().to_string(),
        Err(_) => String::new(),
    };

    // if in /home/USER/... remove that from displayed path
    let path_string = match pwd.strip_prefix(&home) {
        Some(p) => format!("~{}", p),
        None => "".to_string(),
    };
    // width of the terminal
    let size = terminal_size();
    let w: usize = match size {
        Some((Width(ww), Height(_))) => ww.into(),
        None => 0,
    };

    // how much space for padding needs to be added due to emojis being composed of
    // two unicode characters
    let mut emoji_space = 0;
    // get current git branch name
    let mut git_info = git_branch();
    if !git_info.is_empty() {
        git_info = format!("🌿{}", git_info);
        emoji_space += 2;
    }
    // whether on a remote session and if so get user.lastipdigits
    let (user, mut ssh_string) = get_ssh();
    // whether one is logged in as root
    let mut is_root = user.to_lowercase() == "root";
    if !ssh_string.is_empty() {
        ssh_string = format!("{} [{}]", "🛰️", ssh_string);
        emoji_space += 6;
    } else if !user.is_empty() {
        ssh_string = "[ROOT]".to_string();
        is_root = true;
    }

    // construct left side of prompt
    let mut path = format!(
        "{}{} {}",
        color_and_esc(&ssh_string, &shell, {
            if is_root {
                &RED
            } else {
                &GREEN
            }
        }),
        path_string,
        color_and_esc(&git_info, &shell, &GREY),
    );

    // python virutal env names
    let mut py = "".to_string();
    if !conda.is_empty() {
        py = get_filename(conda);
    } else if !venv.is_empty() {
        py = get_filename(venv);
    }

    // projects are all languages that will be tested whether a file exists and whether
    // to display compiler versions
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
                &CARB_ORANGE,
            ),
            "go" => proj_format(
                &go,
                &mut proj_len,
                &mut proj_string,
                &mut emoji_space,
                &shell,
                &TURQUOISE,
            ),
            "py" => proj_format(
                &python,
                &mut proj_len,
                &mut proj_string,
                &mut emoji_space,
                &shell,
                &BLUE,
            ),
            _ => {}
        }
    }

    // length of the left part of the prompt ssh + pwd + space + git branch - compensation
    // for multi unicode emojis
    let path_len = ssh_string.len() + path_string.len() + 1 + git_info.len() - emoji_space;
    // space padding so projects are right aligned
    let mut padding_size = 1;
    if w >= (path_len + proj_len) {
        padding_size = w - path_len - proj_len;
    };
    let padding = vec![" "; padding_size].join("");
    // final construction of the prompt
    path = format!(
        "{}{}{}{}\n{}",
        path,
        padding,
        proj_string,
        color_and_esc(&py, &shell, &BLUE),
        color_and_esc("»", &shell, &ORANGE)
    );
    println!("\r{} ", path);
}
