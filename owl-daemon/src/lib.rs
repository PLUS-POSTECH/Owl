#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;
extern crate tokio;
extern crate owl_exploit;
extern crate owl_rpc;

use self::db::DbPool;
use self::db::models::*;
use self::db::schema::*;
use self::error::Error;
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future;
use tarpc::util::Never;
use tokio::runtime::TaskExecutor;
use owl_rpc::FutureService;

pub mod db;
pub mod error;

#[derive(Clone)]
pub struct OwlDaemon {
    db_pool: DbPool,
    task_executor: TaskExecutor,
}

impl OwlDaemon {
    pub fn new(db_pool: DbPool, task_executor: TaskExecutor) -> OwlDaemon {
        OwlDaemon { db_pool, task_executor }
    }
}

impl FutureService for OwlDaemon {
    type HelloFut = Result<String, Never>;
    fn hello(&self, name: String) -> Self::HelloFut {
        let task_executor = self.task_executor.clone();
        let db_pool = self.db_pool.clone();
        task_executor.spawn(future::lazy(|| {
            Ok(test_db(db_pool).unwrap())
        }));
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

    info!("Delete Test data");
    let delete = diesel::delete(teams::table)
        .filter(teams::name.eq("PLUS"))
        .execute(con)?;
    println!("DELETE: {}", delete);

    Ok(())
}
