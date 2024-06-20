use colored::*;
pub fn color_and_esc(instring: &str, shell: &str, color: CustomColor) -> String {
    let mut s_esc = "\u{5c}\u{5b}";
    let mut e_esc = "\u{5c}\u{5d}";
    match shell {
        "bash" => {}
        "zsh" | "tcsh" => {
            s_esc = "\u{25}\u{7b}";
            e_esc = "\u{25}\u{7d}";
        }
        _ => {
            s_esc = "";
            e_esc = "";
        }
    }
    format!("{s_esc}{}{e_esc}", instring.custom_color(color))
}
