#[macro_use]
extern crate log;

extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate owl_daemon;
extern crate owl_rpc;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;
extern crate tokio;
extern crate tokio_core;

use std::net::ToSocketAddrs;

use dotenv::dotenv;
use owl_daemon::db::build_connection_pool;
use owl_daemon::OwlDaemon;
use owl_rpc::FutureServiceExt;
use tarpc::future::server;
use tokio_core::reactor;

fn main() {
    dotenv().ok();
    env_logger::init();

    let mut reactor = reactor::Core::new().expect("Failed to initialize tokio reactor");
    let task_executor = reactor.runtime().executor();
    let db_pool = build_connection_pool().expect("Failed to connect to the database");
    let (_server_handle, server) = OwlDaemon::new(db_pool, task_executor)
        .listen(
            "localhost:5959".to_socket_addrs().unwrap().next().unwrap(),
            &reactor.handle(),
            server::Options::default(),
        )
        .unwrap();

    info!("Starting Owl Daemon...");
    reactor.run(server).unwrap();
}
