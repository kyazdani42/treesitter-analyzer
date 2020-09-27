use std::collections::HashMap;
use tree_sitter::{Parser, Point, Query, QueryCursor, Tree};

mod utils;

pub use utils::*;

use super::fs::{get_cwd_entries,get_file_content};

pub struct Analyzer {
    language: String,
    query: Query,
    files: HashMap<String, ProjectFile>,
    matches: Vec<Match>,
}

#[derive(Clone)]
pub struct Match {
    pub start_position: Point,
    pub end_position: Point,
    pub file_name: String,
    node_name: String,
    query_name: String,
}

impl Analyzer {
    pub fn new(lang: &str) -> Self {
        let ts_language = get_language(lang).unwrap();
        let query_src = get_query_file(lang);
        let query = Query::new(ts_language, &query_src).unwrap();
        let capture_names = get_capture_names(&query, query_src);

        let files = create_project_files(lang);
        let matches = get_matches(&files, &query, capture_names);

        Self {
            language: lang.to_owned(),
            query,
            files,
            matches,
        }
    }

    pub fn get_definition(&self, file: &str, row: usize, column: usize) -> Option<Match> {
        let cursor_match: Vec<&Match> = self
            .matches
            .iter()
            .filter(|m| {
                let row_included = m.start_position.row <= row && row <= m.end_position.row;
                let col_included =
                    m.start_position.column <= column && column <= m.start_position.column;
                m.file_name == file && row_included && col_included
            })
            .collect();

        println!("{}", cursor_match.len());
        if cursor_match.len() == 0 {
            return None;
        }

        let node_name = cursor_match[0].node_name.to_owned();

        let matches: Vec<&Match> = self
            .matches
            .iter()
            .filter(|m| m.node_name == node_name)
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

fn get_capture_names(query: &Query, query_src: String) -> Vec<String> {
    let start_bytes: Vec<usize> = (0..query.pattern_count())
        .map(|i| query.start_byte_for_pattern(i))
        .collect();

    let mut patterns = vec![];
    for pat_idx in 1..=start_bytes.len() {
        let mut query_src = query_src.clone();
        let start_byte = start_bytes[pat_idx - 1];
        let mut drained: String = if pat_idx != start_bytes.len() {
            query_src.drain(start_byte..start_bytes[pat_idx]).collect()
        } else {
            query_src.drain(start_byte..).collect()
        };
        let query_start = drained.find('@').unwrap() + 1;
        let mut drained: String = drained.drain(query_start..).collect();
        let query_end = drained.find(|c| c == '\n' || c == ' ' || c == ')').unwrap();
        let query_name: String = drained.drain(..query_end).collect();
        patterns.push(query_name);
    }

    patterns
}

fn get_matches(
    files: &HashMap<String, ProjectFile>,
    query: &Query,
    query_names: Vec<String>
) -> Vec<Match> {
    let mut matches = vec![];

    for (filename, project) in files {
        let mut query_cursor = QueryCursor::new();
        let file_content = std::fs::read_to_string(filename).unwrap().to_owned();

        // Dont know what the third argument is for here.
        let query_matches = query_cursor.matches(query, project.tree.root_node(), |_| []);
        query_matches.for_each(|e| {
            let query_name = &query_names[e.pattern_index];
            let capture = e.captures[0];
            let start_byte = capture.node.start_byte();
            let end_byte = capture.node.end_byte();
            let node_name: String = file_content.clone().drain(start_byte..end_byte).collect();
            matches.push(Match {
                node_name,
                file_name: filename.to_owned(),
                query_name: query_name.clone(),
                start_position: capture.node.start_position(),
                end_position: capture.node.end_position(),
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
        let file_content = get_file_content(file);
        let tree = parser.parse(file_content, None).unwrap();
        Self { tree, parser }
    }
}
