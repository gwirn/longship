use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn git_branch() -> String {
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
