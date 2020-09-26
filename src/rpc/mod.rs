use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use jsonrpc_tcp_server::jsonrpc_core::*;
use jsonrpc_tcp_server::*;

use super::Project;

pub type ProjectsRef = Arc<Mutex<HashMap<String, Project>>>;

pub struct Rpc {
    io: IoHandler,
    projects: ProjectsRef,
}

impl Rpc {
    pub fn new() -> Self {
        Self {
            io: IoHandler::default(),
            projects: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn setup(&mut self) {
        self.method_setup();
        self.method_navigation_definition();
    }

    fn method_navigation_definition(&mut self) {
        self.io
            .add_method("navigation/definition", |_params: Params| {
                Ok(Value::String("navigation/definition/success".to_string()))
            });
    }

    fn method_setup(&mut self) {
        let projects = self.projects.clone();
        self.io.add_method("setup", move |params: Params| {
            let params = params.parse::<HashMap<String, String>>().unwrap();
            let language = params.get("language").unwrap();

            if projects.lock().unwrap().iter().any(|(v, _)| v == language) {
                return Ok(Value::String("setup/canceled".to_string()));
            }

            let new_project = Project::new(language);

            projects
                .lock()
                .unwrap()
                .insert(language.to_string(), new_project);

            Ok(Value::String("setup/success".to_string()))
        });
    }

    pub fn run(&mut self) {
        let server = ServerBuilder::new(self.io.clone())
            .start(&"0.0.0.0:7542".parse().unwrap())
            .expect("tcp server could not run at localhost:7542");

        server.wait()
    }
}
