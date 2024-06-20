use nu_ansi_term::Color::Fixed;
pub fn color_and_esc(instring: &str, shell: &str, color: u8) -> String {
    let esc_start: char = '\u{1b}';
    let esc_end_color: char = 'm';

    let mut start_color = "\u{5c}\u{5b}";
    let mut end_color = "\u{5c}\u{5d}";
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
    let final_string: String = Fixed(color)
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
