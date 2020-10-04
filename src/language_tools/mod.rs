use std::env;
use std::fs;

/// Used for finding root of project and getting files
/// For instance:
/// - js/ts -> libs in node_modules, root is package.json, or .git
/// - rust -> libs in a folder on the system, root is Cargo.toml
/// - C -> libs in system folders, in .h files locally, root can be .git, Makefile or something else
pub trait LanguageTools {
    fn get_extensions(&self) -> &[String];
    fn get_project_root(&self) -> &str;

    fn get_files(&self) -> Vec<String> {
        let mut entries = vec![];
        let project_root = self.get_project_root().to_string();
        let extensions = self.get_extensions();
        iterate_entries(&mut entries, extensions, project_root);
        entries
    }
}

fn iterate_entries(entries: &mut Vec<String>, extensions: &[String], cwd: String) {
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

// TODO: create a macro to generate a language tool for a language
struct Rust {
    extensions: Vec<String>,
    project_root: String,
}

impl Rust {
    fn new() -> Self {
        Self {
            extensions: vec!["rs".to_owned()],
            project_root: env::current_dir().unwrap().to_str().unwrap().to_string(),
        }
    }
}

impl LanguageTools for Rust {
    fn get_extensions(&self) -> &[String] {
        &self.extensions
    }

    fn get_project_root(&self) -> &str {
        &self.project_root
    }
}

pub fn get_language_tools(language: &str) -> Option<impl LanguageTools> {
    match language {
        "rust" => Some(Rust::new()),
        _ => None,
    }
}
