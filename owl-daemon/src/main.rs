extern crate diesel;
extern crate futures;
extern crate owl_daemon;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;

use std::thread;

use diesel::prelude::*;
use diesel::result::Error;
use owl_daemon::models::*;
use owl_daemon::schema::*;
use owl_daemon::{connect_db, HelloServer, SyncServiceExt};
use tarpc::sync::server;

fn main() {
    let server_handle = thread::spawn(move || {
        let handle = HelloServer
            .listen("localhost:5959", server::Options::default())
            .unwrap();
        println!("Owl-daemon is starting...");
        handle.run();
    });

    let db_pool = connect_db();

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

    server_handle.join().unwrap();
}
