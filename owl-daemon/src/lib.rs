#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate tarpc;

extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tokio;

use self::db::DbPool;
use self::db::models::*;
use self::db::schema::*;
use self::error::Error;
use diesel::prelude::*;
use diesel::PgConnection;
use tarpc::util::Never;

pub mod db;
pub mod error;

service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
pub struct OwlDaemon {
    db_pool: DbPool,
}

impl OwlDaemon {
    pub fn new(db_pool: DbPool) -> OwlDaemon {
        OwlDaemon { db_pool }
    }
}

impl FutureService for OwlDaemon {
    type HelloFut = Result<String, Never>;
    fn hello(&self, name: String) -> Self::HelloFut {
        test_db(self.db_pool.clone()).unwrap();
        Ok(format!("Hello, {}!", name))
    }
}

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
