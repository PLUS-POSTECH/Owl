#[macro_use]
extern crate log;

extern crate diesel;
extern crate futures;
extern crate owl_daemon;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;
extern crate tokio;

use std::net::{ToSocketAddrs};

use diesel::prelude::*;
use diesel::result::Error;
use owl_daemon::models::*;
use owl_daemon::schema::*;
use owl_daemon::{connect_db, OwlDaemon, FutureServiceExt};
use tarpc::future::server;
use tarpc::tokio_core::reactor;

fn main() {
    let mut reactor = reactor::Core::new().unwrap();
    let (_server_handle, server) = OwlDaemon
        .listen("localhost:5959".to_socket_addrs().unwrap().next().unwrap(),
            &reactor.handle(), server::Options::default())
        .unwrap();

    info!("Connecting DB...")
    let db_pool = connect_db();

    info!("Testing DB...")
    let insert = db_pool
        .run(&|c| -> Result<usize, Error> {
            Ok(diesel::insert_into(teams::table)
                .values((teams::name.eq("PLUS"), teams::description.eq("Best Team")))
                .execute(c)?)
        })
        .unwrap();
    println!("INSERT: {}", insert);

    let fetch = db_pool
        .run(&|c| -> Result<Vec<Team>, Error> {
            Ok(teams::table.load::<Team>(c)?)
        })
        .unwrap();

    for team in fetch {
        println!("FETCH: {} - {}", team.name, team.description);
    }

    let delete = db_pool
        .run(&|c| -> Result<usize, Error> {
            Ok(diesel::delete(teams::table)
                .filter(teams::name.eq("PLUS"))
                .execute(c)?)
        })
        .unwrap();
    println!("DELETE: {}", delete);

    info!("Starting Owl Daemon...")
    reactor.run(server).unwrap();
}
