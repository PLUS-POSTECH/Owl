#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

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
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

use dotenv::dotenv;
use owl_daemon::db::build_connection_pool;
use owl_daemon::error::Error;
use owl_daemon::OwlDaemon;
use owl_rpc::FutureServiceExt;
use tarpc::future::server;
use tarpc::util::FirstSocketAddr;
use tokio_core::reactor;

#[derive(Deserialize)]
struct Config {
    server: Server,
}

#[derive(Deserialize)]
struct Server {
    connection: String,
    db: String,
}

fn read_file_contents(file_name: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(file_name)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main_wrap() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();

    let config: Config = toml::from_slice(&read_file_contents("config.toml")?)?;

    let mut reactor = reactor::Core::new()?;
    let task_executor = reactor.runtime().executor();
    let db_pool = build_connection_pool(config.server.db.clone())?;
    let (_server_handle, server) = OwlDaemon::new(db_pool, task_executor).listen(
        config.server.connection.try_first_socket_addr()?,
        &reactor.handle(),
        server::Options::default(),
    )?;

    info!("Starting Owl Daemon...");
    reactor.run(server)?;

    Ok(())
}

fn main() {
    if let Err(e) = main_wrap() {
        error!("{}", e.to_string());
    }
}
