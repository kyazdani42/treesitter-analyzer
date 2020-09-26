use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use jsonrpc_tcp_server::jsonrpc_core::*;
use jsonrpc_tcp_server::*;

mod project;
use project::Project;

fn main() {
    let mut io = IoHandler::default();
    let projects: Arc<Mutex<HashMap<String, Project>>> = Arc::new(Mutex::new(HashMap::new()));

    let rc_clone = projects.clone();
    io.add_method("setup", move |params: Params| {
        let params = params.parse::<HashMap<String, String>>().unwrap();
        let language = params.get("language").unwrap();

        if rc_clone.lock().unwrap().iter().any(|(v, _)| v == language) {
            return Ok(Value::String("setup/canceled".to_string()))
        }

        let new_project = Project::new(language);

        rc_clone
            .lock()
            .unwrap()
            .insert(language.to_string(), new_project);

        Ok(Value::String("setup/success".to_string()))
    });

    io.add_method("navigation/definition", |_params: Params| {
        Ok(Value::String("navigation/definition/success".to_string()))
    });

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:7542".parse().unwrap())
        .expect("tcp server could not run at localhost:7542");

    server.wait()
}
