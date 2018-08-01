#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate tarpc;

extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tokio;

use self::db::DbPool;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use std::env;
use tarpc::util::Never;

pub mod db;
pub mod models;
pub mod schema;

service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
pub struct OwlDaemon;

impl FutureService for OwlDaemon {
    type HelloFut = Result<String, Never>;
    fn hello(&self, name: String) -> Self::HelloFut {
        Ok(format!("Hello, {}!", name))
    }
}

pub fn connect_db() -> DbPool {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/owl-daemon".to_string());

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    DbPool { pool }
}
