use std::collections::HashMap;
use tree_sitter::{Parser, Query, QueryCursor, Tree};

mod utils;

pub use utils::*;

pub struct Project {
    language: String,
    query: Query,
    files: HashMap<String, ProjectFile>,
    matches: Vec<Match>,
}

pub struct Match {
    start_byte: usize,
    end_byte: usize,
    file_name: String,
    node_name: String,
    query_name: String,
}

impl Project {
    pub fn new(lang: &str) -> Self {
        let ts_language = get_language(lang).unwrap();
        let files = create_project_files(lang);
        let query_src = std::fs::read_to_string(&format!(
            "/home/kiyan/.local/share/treesitter-lsp/queries/{}.scm",
            lang
        ))
        .unwrap()
        .to_string();
        let query = Query::new(ts_language, &query_src).unwrap();
        let mut matches = vec![];
        for (filename, project) in &files {
            let mut query_cursor = QueryCursor::new();
            let query_matches = query_cursor.matches(&query, project.tree.root_node(), |_| []);
            let file_content = std::fs::read_to_string(&filename).unwrap().to_owned();
            let query_names = query.capture_names();
            query_matches.for_each(|e| {
                let query_name = &query_names[e.pattern_index];
                e.captures.iter().for_each(|capture| {
                    let start_byte = capture.node.start_byte();
                    let end_byte = capture.node.end_byte();
                    let node_name: String =
                        file_content.clone().drain(start_byte..end_byte).collect();
                    matches.push(Match {
                        file_name: filename.to_owned(),
                        query_name: query_name.clone(),
                        node_name,
                        start_byte,
                        end_byte,
                    });
                });
            })
        }
        Self {
            language: lang.to_owned(),
            query,
            files,
            matches,
        }
    }
}

pub struct ProjectFile {
    parser: Parser,
    pub tree: Tree,
}

impl ProjectFile {
    pub fn new(lang: &str, file: &str) -> Self {
        let mut parser = get_parser(get_language(lang).unwrap()).unwrap();
        let file_content = std::fs::read_to_string(file).unwrap();
        let tree = parser.parse(file_content, None).unwrap();
        Self { tree, parser }
    }
}
