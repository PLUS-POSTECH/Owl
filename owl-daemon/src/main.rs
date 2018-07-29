extern crate owl_daemon;
extern crate tarpc;

use std::thread;

use tarpc::sync::server;
use owl_daemon::{HelloServer, SyncServiceExt};

fn main() {
    let join_handle = thread::spawn(move || {
        let handle = HelloServer.listen("localhost:5959", server::Options::default())
            .unwrap();
        println!("Owl-daemon is starting...");
        handle.run();
    });

    join_handle.join().unwrap();
}
