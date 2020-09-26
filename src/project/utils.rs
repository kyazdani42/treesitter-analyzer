use tree_sitter::{Language, LanguageError, Parser};

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
