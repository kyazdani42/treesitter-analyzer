mod analyzer;

mod rpc;
use rpc::Rpc;

mod language_tools;

fn main() {
    let mut rpc = Rpc::new("rust");
    rpc.setup();
    rpc.run();
}
