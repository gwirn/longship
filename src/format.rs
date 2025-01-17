use crate::ProjSetting;
use nu_ansi_term::Color::Fixed;
use std::process::Command;

pub const ORANGE: u8 = 208;
pub const GREY: u8 = 245;
pub const BLUE: u8 = 24;
pub const GREEN: u8 = 65;
pub const RED: u8 = 9;
pub const TURQUOISE: u8 = 45;
pub const CARB_ORANGE: u8 = 166;
pub const GOLD1: u8 = 220;
const ESC_START: char = '\u{1b}';
const ESC_END_COLOR: char = 'm';

/// Color given string and surround it with the needed escape characters
///
/// :parameter
/// * `instring`: the string to modify
/// * `shell`: the current shell
/// * `color`: the number representing the color to be used
///     :return
/// * `final_string`: the colored string
pub fn color_and_esc(instring: &str, shell: &str, color: &u8) -> String {
    // which escape codes to be used based on the shell
    let mut start_color = "\u{001}";
    let mut end_color = "\u{002}";
    match shell {
        "bash" => {}
        "zsh" | "tcsh" => {
            start_color = "\u{25}\u{7b}";
            end_color = "\u{25}\u{7d}";
        }
        _ => {
            start_color = "";
            end_color = "";
        }
    }

    // open and close escape codes if it's not done properly
    let mut escaped = false;
    let final_string: String = Fixed(*color)
        .paint(instring)
        .to_string()
        .chars()
        .map(|x| {
            if x == ESC_START && !escaped {
                escaped = true;
                format!("{start_color}{ESC_START}")
            } else if x == ESC_END_COLOR && escaped {
                escaped = false;
                format!("{ESC_END_COLOR}{end_color}")
            } else {
                x.to_string()
            }
        })
        .collect();
    final_string
}

/// Formate the project string (emoji and version and color) based on `ProjSetting`
///
/// :parameter
/// * `settings`: the struct defining all the settings for a given project
/// * `proj_len`: counter for how long the project string got (to calculate the right amount of
///     padding)
/// * `proj_string`: the string with added up project infromation
/// * `emoji_space`: how much the padding needs to be adjusted for multi code emojies
/// * `shell`: the shell currently used
/// * `color`: the 256 color number
///
/// :return
/// * `None`
pub fn proj_format(
    settings: &ProjSetting,
    proj_len: &mut usize,
    proj_string: &mut String,
    shell: &str,
    color: &u8,
) {
    let proj_raw = match Command::new(&settings.compiler)
        .arg(&settings.version_command)
        .output()
    {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(o) => o,
            Err(_) => "".to_string(),
        },
        Err(_) => "".to_string(),
    };
    let proj_split: Vec<_> = proj_raw.trim().split(' ').collect();
    if proj_split.len() == settings.split_len {
        let pre_ps = format!("{}{}", settings.emoji, proj_split[settings.split_idx]);
        *proj_len += pre_ps.len();
        *proj_string = format!("{} {} ", *proj_string, color_and_esc(&pre_ps, shell, color));
    }
}
