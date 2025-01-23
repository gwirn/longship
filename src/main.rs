use file_search::*;
use format::*;
use proj::*;
use rayon::prelude::*;
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
        None => pwd.as_str().to_string(),
    };

    // get current git branch name
    let mut git_info = git_branch();
    if !git_info.is_empty() {
        git_info = format!("{} {}", HERB, git_info);
    }
    // whether on a remote session and if so get user.lastipdigits
    let (user, mut ssh_string) = get_ssh();
    // whether one is logged in as root
    let mut is_root = user.to_lowercase() == "root";
    if !ssh_string.is_empty() {
        ssh_string = format!("{} [{}]", SATELITE, ssh_string);
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

    // projects are all languages that will be tested whether a file exists and whether
    // to display compiler versions
    let proj_string = match is_proj(&pwd, PROJECTS) {
        Some(v) => {
            v.par_iter()
                .map(|ext| match ext.as_str() {
                    "rs" => match proj_format(&rust, &shell, &CARB_ORANGE) {
                        Some(v) => v,
                        None => "".to_owned(),
                    },
                    "zig" => match proj_format(&zig, &shell, &GOLD1) {
                        Some(v) => v,
                        None => "".to_owned(),
                    },
                    "go" => match proj_format(&go, &shell, &TURQUOISE) {
                        Some(v) => v,
                        None => "".to_owned(),
                    },
                    "py" => {
                        // python virutal env names
                        let mut py = "".to_string();
                        if !conda.is_empty() {
                            py = get_filename(conda.clone());
                        } else if !venv.is_empty() {
                            py = get_filename(venv.clone());
                        }
                        match proj_format(&python, &shell, &BLUE) {
                            Some(v) => format!("{} {}", v, color_and_esc(&py, &shell, &BLUE)),
                            None => "".to_owned(),
                        }
                    }
                    _ => "".to_owned(),
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
        None => "".to_string(),
    };
    // final construction of the prompt
    path = format!(
        "{} {}\n{}",
        path,
        proj_string,
        color_and_esc("»", &shell, &ORANGE)
    );
    println!("\r{} ", path);
}
