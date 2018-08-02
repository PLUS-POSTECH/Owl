#[macro_use]
extern crate log;

extern crate diesel;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate owl_daemon;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;
extern crate tokio;
extern crate tokio_core;

use std::net::{ToSocketAddrs};

use owl_daemon::{OwlDaemon, FutureServiceExt};
use owl_daemon::db::{build_connection_pool};
use tarpc::future::server;
use tokio_core::reactor;

fn main() {
    env_logger::init();
    let mut reactor = reactor::Core::new().unwrap();
    let task_executor = reactor.runtime().executor();
    let db_pool = build_connection_pool().expect("Failed to connect to the database");
    let (_server_handle, server) = OwlDaemon::new(db_pool.clone(), task_executor.clone())
        .listen("localhost:5959".to_socket_addrs().unwrap().next().unwrap(),
            &reactor.handle(), server::Options::default())
        .unwrap();

    info!("Starting Owl Daemon...");
    reactor.run(server).unwrap();
}
