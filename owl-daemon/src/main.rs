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
extern crate tokio_threadpool;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

use dotenv::dotenv;
use futures::Future;
use owl_daemon::db::build_connection_pool;
use owl_daemon::error::Error;
use owl_daemon::Config;
use owl_daemon::OwlDaemon;
use owl_rpc::FutureServiceExt;
use tarpc::future::server;
use tarpc::util::FirstSocketAddr;
use tokio::runtime;
use tokio_core::reactor;

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
    let connection_string = config.server.connection.clone();

    let mut threadpool_builder = tokio_threadpool::Builder::new();
    threadpool_builder.max_blocking(config.exploit_config.max_running_exploit_task as usize);
    let runtime = runtime::Builder::new()
        .threadpool_builder(threadpool_builder)
        .build()?;

    let mut reactor = reactor::Core::new()?;
    let db_pool = build_connection_pool(config.server.db.clone())?;
    let (_server_handle, server) = OwlDaemon::new(db_pool, runtime.executor(), config).listen(
        connection_string.try_first_socket_addr()?,
        &reactor.handle(),
        server::Options::default().max_payload_size(32_000_000),
    )?;

    info!("Starting Owl Daemon...");
    reactor.run(server)?;
    info!("RPC shut down...");
    info!("Waiting exploits...");
    runtime.shutdown_on_idle().wait()?;
    info!("Shutting down...");

    Ok(())
}

fn main() {
    if let Err(e) = main_wrap() {
        error!("{}", e.to_string());
    }
}
