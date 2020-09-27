use tree_sitter::{Language, LanguageError, Parser};

use crate::fs::get_file_content;

pub fn get_extensions(lang: &str) -> Option<Vec<&str>> {
    match lang {
        "rust" => Some(vec!["rs"]),
        "lua" => Some(vec!["lua"]),
        _ => None,
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

pub fn get_query_file(language: &str) -> String {
    let base_dirs = xdg::BaseDirectories::new().unwrap();
    let data_home = base_dirs.get_data_home();
    let xdg_folder = data_home.to_str().unwrap();
    let query_file = format!("{}/treesitter-lsp/queries/{}.scm", xdg_folder, language);
    get_file_content(&query_file)
}
