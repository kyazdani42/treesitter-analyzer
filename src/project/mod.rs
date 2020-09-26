use std::collections::HashMap;
use tree_sitter::{Parser, Query, QueryCursor, Tree};

mod fs;
mod utils;

pub use fs::*;
pub use utils::*;

pub struct Project {
    language: String,
    query: Query,
    files: HashMap<String, ProjectFile>,
    matches: Vec<Match>,
}

#[derive(Clone)]
pub struct Match {
    pub start_byte: usize,
    pub end_byte: usize,
    pub file_name: String,
    node_name: String,
    query_name: String,
}

impl Project {
    pub fn new(lang: &str) -> Self {
        let ts_language = get_language(lang).unwrap();
        let query_src = get_query_file(lang);
        let query = Query::new(ts_language, &query_src).unwrap();

        let files = create_project_files(lang);
        let matches = get_matches(&files, &query);

        Self {
            language: lang.to_owned(),
            query,
            files,
            matches,
        }
    }

    pub fn get_definition(&self, name: &str) -> Option<Match> {
        let matches: Vec<&Match> = self
            .matches
            .iter()
            .filter(|m| m.node_name == name)
            .collect();

        let def = matches
            .iter()
            .find(|m| m.query_name == "definition.exported");
        if let Some(def) = def {
            return Some((*def).clone());
        }

        let def = matches.iter().find(|m| m.query_name == "definition.scoped");
        if let Some(def) = def {
            Some((*def).clone())
        } else {
            None
        }
    }
}

fn get_matches(files: &HashMap<String, ProjectFile>, query: &Query) -> Vec<Match> {
    let mut matches = vec![];

    for (filename, project) in files {
        let mut query_cursor = QueryCursor::new();
        let file_content = std::fs::read_to_string(filename).unwrap().to_owned();
        let query_names = query.capture_names();

        let query_matches = query_cursor.matches(query, project.tree.root_node(), |_| []);
        query_matches.for_each(|e| {
            let query_name = &query_names[e.pattern_index];
            e.captures.iter().for_each(|capture| {
                let start_byte = capture.node.start_byte();
                let end_byte = capture.node.end_byte();
                let node_name: String = file_content.clone().drain(start_byte..end_byte).collect();
                matches.push(Match {
                    file_name: filename.to_owned(),
                    query_name: query_name.clone(),
                    node_name,
                    start_byte,
                    end_byte,
                });
            });
        });
    }

    matches
}

fn create_project_files(lang: &str) -> HashMap<String, ProjectFile> {
    let extensions = get_extensions(lang).unwrap();
    let entries = get_cwd_entries(&extensions);
    let mut files = HashMap::new();

    for file in entries {
        files.insert(file.to_owned(), ProjectFile::new(lang, &file));
    }

    files
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
