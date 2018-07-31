extern crate actix;
extern crate diesel;
extern crate futures;
extern crate owl_daemon;
extern crate tarpc;

use std::sync::Arc;
use std::thread;

use futures::{future, Future};
use tarpc::sync::server;
use owl_daemon::{HelloServer, SyncServiceExt, connect_db};

fn main() {
    let server_handle = thread::spawn(move || {
        let handle = HelloServer.listen("localhost:5959", server::Options::default())
            .unwrap();
        println!("Owl-daemon is starting...");
        handle.run();
    });

    server_handle.join().unwrap();
}
