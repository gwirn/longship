use crate::proj_version;
use crate::ProjSetting;
use nu_ansi_term::Color::Fixed;
pub fn color_and_esc(instring: &str, shell: &str, color: &u8) -> String {
    let esc_start: char = '\u{1b}';
    let esc_end_color: char = 'm';

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

    let mut escaped = false;
    let final_string: String = Fixed(*color)
        .paint(instring)
        .to_string()
        .chars()
        .map(|x| {
            if x == esc_start && !escaped {
                escaped = true;
                format!("{start_color}{esc_start}")
            } else if x == esc_end_color && escaped {
                escaped = false;
                format!("{esc_end_color}{end_color}")
            } else {
                x.to_string()
            }
        })
        .collect();
    final_string
}
pub fn proj_format(
    settings: &ProjSetting,
    proj_len: &mut usize,
    proj_string: &mut String,
    emoji_space: &mut usize,
    shell: &str,
    color: &u8,
) {
    let proj_raw = proj_version(&settings.compiler, &settings.version_command);
    let proj_split: Vec<_> = proj_raw.trim().split(' ').collect();
    if proj_split.len() == settings.split_len {
        let pre_ps = format!("{}{}", settings.emoji, proj_split[settings.split_idx]);
        *proj_len += pre_ps.len();
        *proj_string = format!("{} {} ", *proj_string, color_and_esc(&pre_ps, shell, color));
        *emoji_space += settings.emoji_space_add
    }
}
