use file_search::*;
use format::*;
use proj::*;
use std::env;
use utils::*;

mod file_search;
mod format;
mod proj;
mod utils;

fn main() {
    // get the setting struct for all supported projects
    let (python, zig, rust, go) = get_proj_settings();
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
        None => (&*pwd.to_string()).into(),
    };

    // how much space for padding needs to be added due to emojis being composed of
    // two unicode characters
    // get current git branch name
    let mut git_info = git_branch();
    if !git_info.is_empty() {
        git_info = format!("🌿{}", git_info);
    }
    // whether on a remote session and if so get user.lastipdigits
    let (user, mut ssh_string) = get_ssh();
    // whether one is logged in as root
    let mut is_root = user.to_lowercase() == "root";
    if !ssh_string.is_empty() {
        ssh_string = format!("{} [{}]", "🛰️", ssh_string);
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
            "rs" => proj_format(&rust, &mut proj_len, &mut proj_string, &shell, &CARB_ORANGE),
            "zig" => proj_format(&zig, &mut proj_len, &mut proj_string, &shell, &GOLD1),
            "go" => proj_format(&go, &mut proj_len, &mut proj_string, &shell, &TURQUOISE),
            "py" => proj_format(&python, &mut proj_len, &mut proj_string, &shell, &BLUE),
            _ => {}
        }
    }

    // final construction of the prompt
    path = format!(
        "{}{}{}\n{}",
        path,
        proj_string,
        color_and_esc(&py, &shell, &BLUE),
        color_and_esc("»", &shell, &ORANGE)
    );
    println!("\r{} ", path);
}
