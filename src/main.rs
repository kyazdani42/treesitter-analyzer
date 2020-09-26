use tree_sitter::{Language, LanguageError, Parser};

mod rpc;

fn main() {
    let file = std::path::Path::new("/home/kiyan/dev/other/treesitter-lsp/src/main.rs");
    let extension = file.extension().unwrap().to_str().unwrap();
    let file = std::fs::read_to_string(file).unwrap();

    let language = extension_to_language(extension).unwrap();
    let mut parser = get_parser(language).unwrap();

    let tree = parser.parse(&file, None).unwrap();
    println!("{}", tree.walk().node().is_named());

    rpc::run_server();
}

pub fn get_parser(language: Language) -> Result<Parser, LanguageError> {
    let mut parser = Parser::new();
    parser.set_language(language)?;
    Ok(parser)
}

extern "C" {
    fn tree_sitter_rust() -> Language;
}

pub fn extension_to_language(extension: &str) -> Option<Language> {
    match extension {
        "rs" => Some(unsafe { tree_sitter_rust() }),
        _ => None,
    }
}
