extern crate owl_daemon;
extern crate tarpc;
extern crate diesel;

use std::thread;

use diesel::prelude::*;
use tarpc::sync::server;
use owl_daemon::{HelloServer, SyncServiceExt, connect_db, models, schema};

fn main() {
    let server_handle = thread::spawn(move || {
        let handle = HelloServer.listen("localhost:5959", server::Options::default())
            .unwrap();
        println!("Owl-daemon is starting...");
        handle.run();
    });

    let db_handle = thread::spawn(move || {
        use schema::teams;
        use models::Team;

        let pg_connection = connect_db();
        diesel::insert_into(teams::table)
            .values((teams::name.eq("PLUS"), teams::description.eq("Best Team")))
            .execute(&pg_connection)
            .unwrap();

        let results = teams::table
            .load::<Team>(&pg_connection)
            .unwrap();

        for team in results {
            println!("{}: {}", team.name, team.description)
        }

        diesel::delete(teams::table)
            .filter(teams::name.eq("PLUS"))
            .execute(&pg_connection)
            .unwrap();
    });

    db_handle.join().unwrap();
    server_handle.join().unwrap();
}
