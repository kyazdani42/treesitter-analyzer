use std::env;
use std::fs;

pub fn get_cwd_entries(extensions: &[&str]) -> Vec<String> {
    let cwd = env::current_dir().unwrap();
    let cwd = cwd.to_str().unwrap().to_owned();

    let mut entries = vec![];
    iterate_entries(&mut entries, extensions, cwd);

    entries
}

fn iterate_entries(entries: &mut Vec<String>, extensions: &[&str], cwd: String) {
    for entry in fs::read_dir(cwd).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension();
        let filepath = path.to_str().unwrap().to_owned();
        let filetype = entry.file_type().unwrap();

        if filetype.is_dir() {
            iterate_entries(entries, extensions, filepath);
        } else if filetype.is_file() && extension.is_some() {
            let extension = extension.unwrap().to_str().unwrap();
            let is_project_file = extensions.iter().any(|ext| extension == *ext);
            if is_project_file {
                entries.push(filepath);
            }
        }
    }
}
