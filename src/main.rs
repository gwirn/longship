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
        Some(p) => {
            let mut ps = p.to_owned();
            ps.insert(0, '~');
            ps
        }
        None => pwd.clone(),
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
    let path = format!(
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
    let (n_proj_raw, proj_string_raw): (Vec<_>, Vec<_>) = match is_proj(&pwd, PROJECTS) {
        Some(v) => {
            v.par_iter()
                .map(|ext| match ext.as_str() {
                    "rs" => match proj_format(&rust, &shell, &CARB_ORANGE) {
                        Some(v) => (2, v),
                        None => (0, "".to_owned()),
                    },
                    "zig" => match proj_format(&zig, &shell, &GOLD1) {
                        Some(v) => (0, v),
                        None => (0, "".to_owned()),
                    },
                    "go" => match proj_format(&go, &shell, &TURQUOISE) {
                        Some(v) => (2, v),
                        None => (0, "".to_owned()),
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
                            Some(mut v) => {
                                v.push(' ');
                                v.push_str(color_and_esc(&py, &shell, &BLUE).as_str());
                                (2, v)
                            }
                            None => (0, "".to_owned()),
                        }
                    }
                    _ => (0, "".to_owned()),
                })
                .collect()
        }
        None => (vec![0], vec!["".to_string()]),
    };
    let proj_string = proj_string_raw.join(" ");
    let exec_time = command_time("LONGSHIP_TIME_STAMP", 2);
    let exec_ret = command_retun("LONGSHIP_RET_CODE");
    let mut exec_string: String = "".to_string();
    if let Some((h, m, s)) = exec_time {
        if let Some(e_r) = exec_ret {
            let mut ret_color = GREEN;
            let mut ret_mark = CHECKMARK;
            if !e_r {
                ret_color = RED;
                ret_mark = CROSSMARK;
            }
            let mut time_str = "".to_string();
            if s > 0 {
                time_str = format!("{}s", s)
            }
            if m > 0 {
                time_str = format!("{}m{}", m, time_str)
            }
            if h > 0 {
                time_str = format!("{}h{}", h, time_str)
            }
            let mut mark_buf: [u8; 4] = [0; 4];
            let mark_str: &str = ret_mark.encode_utf8(&mut mark_buf);
            time_str.insert_str(0, mark_str);
            exec_string = color_and_esc(&time_str, &shell, &ret_color);
        }
    }
    let mut padding = "".to_string();
    if exec_string.len() > 0 {
        padding = match screensize() {
            Some((_, x)) => match gen_re() {
                Ok((ansi_re, unicode_re, all_unicode_re)) => {
                    let prompt_len = raw_len(&path, &ansi_re, &unicode_re)
                        + 2
                        + raw_len(&proj_string, &ansi_re, &unicode_re)
                        - raw_len(&proj_string, &ansi_re, &all_unicode_re)
                        + n_proj_raw.iter().sum::<usize>();
                    let padding_size = {
                        if prompt_len > x {
                            1
                        } else {
                            x - prompt_len - raw_len(&exec_string, &ansi_re, &unicode_re)
                        }
                    };
                    vec![" "; padding_size].join("")
                }
                Err(_) => " ".to_string(),
            },
            None => " ".to_string(),
        };
    }

    // final construction of the prompt
    let prompt = format!(
        "{} {}{}{}\n{}",
        path,
        proj_string,
        padding,
        exec_string,
        color_and_esc("»", &shell, &ORANGE)
    );

    println!("\r{} ", prompt);
}
