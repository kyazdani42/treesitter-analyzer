use std::collections::HashMap;
use std::env;
use std::fs;

use tree_sitter::{Language, LanguageError, Parser};

use super::ProjectFile;

type FnameToProjectFile = HashMap<String, ProjectFile>;

pub fn create_project_files(lang: &str) -> FnameToProjectFile {
    let extensions = get_extensions(lang).unwrap();
    let entries = get_cwd_entries(&extensions);
    let mut files = HashMap::new();

    for file in entries {
        files.insert(file.to_owned(), ProjectFile::new(lang, &file));
    }

    files
}

fn get_extensions(lang: &str) -> Option<Vec<&str>> {
    match lang {
        "rust" => Some(vec!["rs"]),
        "lua" => Some(vec!["lua"]),
        _ => None,
    }
}

fn get_cwd_entries(extensions: &[&str]) -> Vec<String> {
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

pub fn get_parser(language: Language) -> Result<Parser, LanguageError> {
    let mut parser = Parser::new();
    parser.set_language(language)?;
    Ok(parser)
}

extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_lua() -> Language;
}

pub fn get_language(language: &str) -> Option<Language> {
    match language {
        "rust" => Some(unsafe { tree_sitter_rust() }),
        "lua" => Some(unsafe { tree_sitter_lua() }),
        _ => None,
    }
}
