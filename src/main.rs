use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::usize;
use terminal_size::{terminal_size, Height, Width};

fn get_filename(inpath: String) -> String {
    let path = Path::new(&inpath);
    match path.file_name() {
        Some(p) => match p.to_os_string().into_string() {
            Ok(pp) => pp,
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    }
}
fn get_ssh() -> String {
    let user = match env::var("USER") {
        Ok(u) => u,
        Err(_) => "EXT".to_string(),
    };
    let ip_en = match env::var("SSH_CONNECTION") {
        Ok(i) => {
            let sshcon_split = i.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
            if sshcon_split.len() == 4 {
                let ip_split = sshcon_split[2]
                    .split('.')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                format!(".{}", ip_split[ip_split.len() - 1])
            } else {
                "".to_string()
            }
        }
        Err(_) => "".to_string(),
    };
    if env::var("SSH_CONNECTION").is_ok() {
        return format!("{}{}", user, ip_en);
    }
    if env::var("SSH_CLIENT").is_ok() {
        return format!("{}{}", user, ip_en);
    }
    if env::var("SSH_TTY").is_ok() {
        return format!("{}{}", user, ip_en);
    }
    "".to_string()
}
fn git_branch() -> String {
    // get path of the current directory
    let mut cur_loc = match env::current_dir() {
        Ok(cloc) => cloc,
        Err(..) => return "".into(),
    };
    let cur_loc_inmut = cur_loc.clone();
    //  iteratively remove one component of the path and check if .git/HEAD is present
    for _c in cur_loc_inmut.components() {
        let file_path: PathBuf = [cur_loc.clone(), ".git".into(), "HEAD".into()]
            .iter()
            .collect();
        if file_path.exists() {
            // read symbolic reference to branch currently on
            let contents = match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(..) => return "".into(),
            };
            let content_path: &Vec<&str> = &contents.split(' ').collect();
            // get filename and remove newline so it can be printed without disturbing PS1
            let branch_name = match Path::new(&content_path[content_path.len() - 1]).file_name() {
                Some(name) => match name.to_str() {
                    Some(namestring) => match namestring.strip_suffix('\n') {
                        Some(final_name) => final_name,
                        None => return "".into(),
                    },
                    None => return "".into(),
                },
                None => return "".into(),
            };
            return branch_name.to_string();
        }
        cur_loc.pop();
    }
    "".into()
}
fn main() {
    let orange = CustomColor::new(255, 135, 0);
    let grey = CustomColor::new(138, 138, 138);
    let blue = CustomColor::new(0, 95, 135);
    let silver = CustomColor::new(192, 192, 192);

    let home = match env::var("HOME") {
        Ok(p) => p,
        Err(_) => "".to_string(),
    };
    let conda = match env::var("CONDA_PREFIX") {
        Ok(p) => p,
        Err(_) => "".to_string(),
    };
    let venv = match env::var("VIRTUAL_ENV") {
        Ok(p) => p,
        Err(_) => "".to_string(),
    };

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

    let mut py = "".to_string();
    if !conda.is_empty() {
        py = get_filename(conda);
    } else if !venv.is_empty() {
        py = get_filename(venv);
    }

    let mut emoji_space = 0;
    let mut git_info = git_branch();
    if !git_info.is_empty() {
        git_info = format!("🌿{}", git_info);
        emoji_space += 2;
    }
    let mut ssh_string = get_ssh();
    if !ssh_string.is_empty() {
        ssh_string = format!("{} [{}]", "🛰️", get_ssh());
        emoji_space += 6;
    }

    let mut path = format!(
        "{}{}{} {}",
        "┏━".custom_color(grey),
        ssh_string.custom_color(silver),
        path_string,
        git_info.custom_color(grey)
    );
    let path_len = 2 + ssh_string.len() + path_string.len() + 1 + git_info.len() - emoji_space;
    let pylen = py.len();

    let mut padding_size = 1;
    if w >= (path_len + pylen) {
        padding_size = w - path_len - pylen;
    };
    let padding = vec![" "; padding_size].join("");
    path = format!(
        "{}{}{}\n{}{}",
        path,
        padding,
        py.custom_color(blue),
        "┗━".custom_color(grey),
        "❱".custom_color(orange)
    );
    println!("{}", path);
}
