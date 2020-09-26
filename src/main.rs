mod project;
use project::Project;

mod rpc;
use rpc::Rpc;

fn main() {
    let mut rpc = Rpc::new();
    rpc.setup();
    rpc.run();
}
