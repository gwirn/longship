use std::env;
use std::path::Path;

pub const SATELITE: char = '\u{1F6F0}';
pub const HERB: char = '\u{1F33F}';
pub const SNAKE: char = '\u{1F40D}';
pub const CRAB: char = '\u{1F980}';
pub const CROCODILE: char = '\u{1F40A}';
pub const CHIPMUNK: char = '\u{1F43F}';

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
