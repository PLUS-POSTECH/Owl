#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

extern crate chrono;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate tarpc;

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use tarpc::util::Never;

pub mod models;
pub mod schema;

service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
pub struct HelloServer;

impl SyncService for HelloServer {
    fn hello(&self, name: String) -> Result<String, Never> {
        Ok(format!("Hello, {}!", name))
    }
}

pub fn connect_db() -> PgConnection {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_e) => "postgres://postgres@localhost/owl-daemon".to_string(),
    };

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
