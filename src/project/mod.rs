use std::collections::HashMap;
use tree_sitter::{Parser, Tree};

mod utils;

pub use utils::*;

pub struct Project {
    language: String,
    files: HashMap<String, ProjectFile>,
}

impl Project {
    pub fn new(lang: &str) -> Self {
        Self {
            language: lang.to_owned(),
            files: create_project_files(lang),
        }
    }
}

pub struct ProjectFile {
    parser: Parser,
    tree: Tree,
    nodes: Vec<Node>,
}

impl ProjectFile {
    pub fn new(lang: &str, file: &str) -> Self {
        let mut parser = get_parser(get_language(lang).unwrap()).unwrap();
        let file_content = std::fs::read_to_string(file).unwrap();
        let tree = parser.parse(file_content, None).unwrap();
        let nodes = iter_tree(&tree);
        Self {
            tree,
            parser,
            nodes,
        }
    }
}

pub struct Node {
    name: String,
    _type: String,
    definition: bool,
    range: Range,
}

pub struct Range {
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
}
