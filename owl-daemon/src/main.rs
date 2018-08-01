extern crate diesel;
extern crate failure;
extern crate futures;
extern crate owl_daemon;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;

use std::thread;

use diesel::prelude::*;
use diesel::PgConnection;
use owl_daemon::db::models::*;
use owl_daemon::db::schema::*;
use owl_daemon::db::{build_connection_pool, DbPool};
use owl_daemon::error::Error;
use owl_daemon::{HelloServer, SyncServiceExt};
use tarpc::sync::server;

fn test_db(db_pool: DbPool) -> Result<(), Error> {
    let con: &PgConnection = &*db_pool.get()?;

    let insert = diesel::insert_into(teams::table)
        .values((teams::name.eq("PLUS"), teams::description.eq("Best Team")))
        .execute(con)?;
    println!("INSERT: {}", insert);

    let fetch = teams::table.load::<Team>(con)?;
    for team in fetch {
        println!("FETCH: {} - {}", team.name, team.description);
    }

    let delete = diesel::delete(teams::table)
        .filter(teams::name.eq("PLUS"))
        .execute(con)?;
    println!("DELETE: {}", delete);

    Ok(())
}

fn main() {
    let server_handle = thread::spawn(move || {
        let handle = HelloServer
            .listen("localhost:5959", server::Options::default())
            .unwrap();
        println!("Owl-daemon is starting...");
        handle.run();
    });

    let db_pool = build_connection_pool().expect("Failed to connect to the database");
    test_db(db_pool.clone()).expect("DB test failed");

    server_handle.join().unwrap();
}
