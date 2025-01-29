use std::{
    env,
    path::Path,
    process::{Command, Stdio},
    time::SystemTime,
};

pub const SATELITE: char = '\u{1F6F0}';
pub const HERB: char = '\u{1F33F}';
pub const SNAKE: char = '\u{1F40D}';
pub const CRAB: char = '\u{1F980}';
pub const VOLTAGE: char = '\u{26A1}';
pub const CHIPMUNK: char = '\u{1F43F}';
pub const CHECKMARK: char = '\u{2714}';
pub const CROSSMARK: char = '\u{0078}';
pub const LINK: char = '\u{1F517}';

/// get the current shell
///
/// :parameter
/// * `None`
///
/// :return
/// * `stem`: name of the current shell as per its path or `""`
pub fn get_shell() -> String {
    let shell_path = get_env_var("SHELL");
    match Path::new(&shell_path).file_stem() {
        Some(pre_stem) => match pre_stem.to_os_string().into_string() {
            Ok(stem) => stem,
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    }
}

/// get an environment variable
///
/// :parameter
/// * `variable`: the variable to look for
///
/// :return
/// * `p`: the value of the variable or `""`
pub fn get_env_var(variable: &str) -> String {
    match env::var(variable) {
        Ok(p) => p,
        Err(_) => "".to_string(),
    }
}

/// get filename of a given path
///
/// :parameter
/// * `inpath`: the file path to extract from
///
/// :return
/// * `pp`: the file name or `""`
pub fn get_filename(inpath: String) -> String {
    let path = Path::new(&inpath);
    match path.file_name() {
        Some(p) => match p.to_os_string().into_string() {
            Ok(pp) => pp,
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    }
}

/// get whether it is a ssh session or not from environment variable
///
/// :parameter
/// * `None`
///     :return
/// * (`user`, `ip_en`): the user name and the user name with the last two digits of the IP address
pub fn get_ssh() -> (String, String) {
    let user = get_env_var("USER");
    let ip_en = match env::var("SSH_CONNECTION") {
        Ok(i) => {
            let sshcon_split = i.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
            if sshcon_split.len() == 4 {
                let ip_split = sshcon_split[2]
                    .split('.')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                if !ip_split.is_empty() {
                    format!(".{}", ip_split[ip_split.len() - 1])
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        }
        Err(_) => "".to_string(),
    };
    if env::var("SSH_CONNECTION").is_ok() {
        return (user.clone(), format!("{}{}", user, ip_en));
    }
    if env::var("SSH_CLIENT").is_ok() {
        return (user.clone(), format!("{}{}", user, ip_en));
    }
    if env::var("SSH_TTY").is_ok() {
        return (user.clone(), format!("{}{}", user, ip_en));
    }
    ("".to_string(), "".to_string())
}

pub fn command_time(time_key: &str, threshold: u64) -> Option<(u64, u64, u64)> {
    let now = SystemTime::now();
    let since_epoch = match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(se) => se.as_secs(),
        Err(_) => return None,
    };
    match env::var(time_key) {
        Ok(c_time) => match c_time.parse::<u64>() {
            Ok(t) => {
                if since_epoch > t {
                    let time_diff = since_epoch - t;
                    if time_diff >= threshold {
                        let hours = (time_diff / 3600) % 24;
                        let minutes = (time_diff / 60) % 60;
                        let seconds = time_diff % 60;
                        Some((hours, minutes, seconds))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub fn command_retun(cmd_key: &str) -> Option<bool> {
    match env::var(cmd_key) {
        Ok(c) => match c.as_str() {
            "0" => Some(true),
            _ => Some(false),
        },
        Err(_) => None,
    }
}

pub fn _screensize() -> Option<(usize, usize)> {
    let size_out = if cfg!(target_os = "linux") {
        Command::new("stty")
            .arg("size")
            .arg("-F")
            .arg("/dev/stderr")
            .stderr(Stdio::inherit())
            .output()
    } else {
        Command::new("stty")
            .arg("-f")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
    };
    match size_out {
        Ok(s_str) => match String::from_utf8(s_str.stdout) {
            Ok(v) => {
                let mut data = v.split_whitespace();
                let rows = match {
                    match data.next() {
                        Some(r) => r,
                        None => return None,
                    }
                }
                .parse::<usize>()
                {
                    Ok(rv) => rv,
                    Err(_) => return None,
                };
                let cols = match {
                    match data.next() {
                        Some(r) => r,
                        None => return None,
                    }
                }
                .parse::<usize>()
                {
                    Ok(rv) => rv,
                    Err(_) => return None,
                };
                Some((rows, cols))
            }
            Err(_) => None,
        },
        Err(_) => None,
    }
}
