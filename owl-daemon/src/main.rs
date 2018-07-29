extern crate owl_daemon;
extern crate tarpc;

use std::sync::mpsc;
use std::thread;

use tarpc::sync::{client, server,
                  client::ClientExt};
use owl_daemon::{
    HelloServer,
    SyncClient,
    SyncServiceExt,
};

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let handle = HelloServer.listen("localhost:0", server::Options::default())
            .unwrap();
        tx.send(handle.addr()).unwrap();
        handle.run();
    });
    let addr = rx.recv().unwrap();
    println!("{:?}", addr);
    let client = SyncClient::connect(addr, client::Options::default()).unwrap();
    println!("{}", client.hello("Mom".to_string()).unwrap());
}
