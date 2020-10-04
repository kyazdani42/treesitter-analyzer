use tree_sitter::{Language, LanguageError, Node, Parser};

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
    let query_file = format!("{}/treesitter-analyzer/queries/{}.scm", xdg_folder, language);
    get_file_content(&query_file)
}

pub fn smallest_node_at_point(node: Node, row: usize, column: usize) -> Node {
    let mut cursor = node.walk();
    let mut next_child = node;

    loop {
        if next_child.named_child_count() == 0 {
            break;
        }
        for child in next_child.named_children(&mut cursor) {
            let start_pos = child.start_position();
            let end_pos = child.end_position();
            if start_pos.row == row && end_pos.row == row {
                if start_pos.column <= column && column <= end_pos.column {
                    next_child = child;
                }
            } else if start_pos.row <= row && row <= end_pos.row {
                next_child = child;
            }
        }
    }

    next_child
}

pub fn get_file_content(file: &str) -> String {
    std::fs::read_to_string(file).unwrap().to_string()
}
