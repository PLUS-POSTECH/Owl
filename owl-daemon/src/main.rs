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

use diesel::prelude::*;
use diesel::PgConnection;
use owl_daemon::{OwlDaemon, FutureServiceExt};
use owl_daemon::db::models::*;
use owl_daemon::db::schema::*;
use owl_daemon::db::{build_connection_pool, DbPool};
use owl_daemon::error::Error;
use tarpc::future::server;
use tokio_core::reactor;

fn test_db(db_pool: DbPool) -> Result<(), Error> {
    info!("Testing DB...");
    let con: &PgConnection = &*db_pool.get()?;

    info!("Insert Test data");
    let insert = diesel::insert_into(teams::table)
        .values((teams::name.eq("PLUS"), teams::description.eq("Best Team")))
        .execute(con)?;
    println!("INSERT: {}", insert);

    info!("Fetch Test data");
    let fetch = teams::table.load::<Team>(con)?;
    for team in fetch {
        println!("FETCH: {} - {}", team.name, team.description);
    }

    info!("Insert Test data");
    let delete = diesel::delete(teams::table)
        .filter(teams::name.eq("PLUS"))
        .execute(con)?;
    println!("DELETE: {}", delete);

    Ok(())
}

fn main() {
    env_logger::init();
    let mut reactor = reactor::Core::new().unwrap();
    let db_pool = build_connection_pool().expect("Failed to connect to the database");
    let (_server_handle, server) = OwlDaemon::new(db_pool.clone())
        .listen("localhost:5959".to_socket_addrs().unwrap().next().unwrap(),
            &reactor.handle(), server::Options::default())
        .unwrap();

    test_db(db_pool.clone()).expect("DB test failed");

    info!("Starting Owl Daemon...");
    reactor.run(server).unwrap();
}
