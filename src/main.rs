use std::sync::Arc;

mod lib;

fn main() {
    let mut signal = lib::RBXScriptSignal::new();

    signal.connect_once(Arc::new(Box::new(|| {
        println!("Hello, world!");
    })));

    signal.fire();
    signal.fire();
    signal.fire();

    signal.connect(Arc::new(Box::new(|| {
        println!("Hello, world!");
    })));

    signal.fire();
    signal.fire();
    signal.fire();
}
