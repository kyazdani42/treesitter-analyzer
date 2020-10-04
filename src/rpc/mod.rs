use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use jsonrpc_tcp_server::jsonrpc_core::*;
use jsonrpc_tcp_server::*;

use super::analyzer::Analyzer;

pub struct Rpc {
    io: IoHandler,
    analyzer: Arc<Mutex<Analyzer>>,
}

impl Rpc {
    pub fn new(language: &str) -> Self {
        Self {
            io: IoHandler::default(),
            analyzer: Arc::new(Mutex::new(Analyzer::new(language))),
        }
    }

    pub fn setup(&mut self) {
        self.method_navigation_definition();
    }

    // row : is 0 based
    // column : is 0 based
    // file : absolute path
    fn method_navigation_definition(&mut self) {
        let analyzer = Arc::clone(&self.analyzer);
        self.io
            .add_method("navigation/definition", move |params: Params| {
                let params = params.parse::<HashMap<String, String>>().unwrap();

                let file = params.get("file").unwrap();
                let row = params.get("row").unwrap().parse::<usize>().unwrap();
                let column = params.get("column").unwrap().parse::<usize>().unwrap();

                let definition = analyzer.lock().unwrap().get_definition(file, row, column);

                if let Some(definition) = definition {
                    let result = format!(
                        r#"{{"row":"{}","col":"{}","file":"{}"}}"#,
                        definition.start_position.row,
                        definition.start_position.column,
                        definition.file_name
                    );
                    Ok(Value::String(result))
                } else {
                    Ok(Value::String("navigation/definition/error".to_string()))
                }
            });
    }

    pub fn run(&mut self) {
        let server = ServerBuilder::new(self.io.clone())
            .start(&"0.0.0.0:7542".parse().unwrap())
            .expect("tcp server could not run at localhost:7542");

        server.wait()
    }
}
