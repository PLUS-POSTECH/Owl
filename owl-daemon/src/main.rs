extern crate actix;
extern crate diesel;
extern crate futures;
extern crate owl_daemon;
extern crate tarpc;

use std::sync::Arc;
use std::thread;

use actix::{Arbiter, SyncArbiter};
use futures::{future, Future};
use tarpc::sync::server;
use owl_daemon::{HelloServer, SyncServiceExt, connect_db};
use owl_daemon::actors::{CreateTeam, GetTeam, DeleteTeam, DbExecutor};

fn main() {
    let system = actix::System::new("owl-daemon");
    let server_handle = thread::spawn(move || {
        let handle = HelloServer.listen("localhost:5959", server::Options::default())
            .unwrap();
        println!("Owl-daemon is starting...");
        handle.run();
    });

    let addr = Arc::new(SyncArbiter::start(1, move || DbExecutor {db_connection: connect_db()}));


    let create_message = CreateTeam {
        name: "PLUS".to_string(),
        description: "Best Team".to_string(),
    };

    let create_res = addr.clone().send(create_message);
    let get_addr = addr.clone();
    let delete_addr = addr.clone();
    Arbiter::spawn(create_res.then(move |res| {
        let id = res.unwrap().unwrap();
        println!("CREATE: {}", id);
        get_addr.send(GetTeam)
    }).then(move |res| {
        let teams = res.unwrap().unwrap();
        println!("Fetch");
        for team in teams {
            println!("{}: {}", team.name, team.description);
        }
        println!("-----");
        let delete_message = DeleteTeam {
            name: "PLUS".to_string(),
        };
        delete_addr.send(delete_message)
    }).then(move |res| {
        let id = res.unwrap().unwrap();
        println!("DELETE: {}", id);
        future::result(Ok(()))
    }));

    system.run();
    server_handle.join().unwrap();
}
