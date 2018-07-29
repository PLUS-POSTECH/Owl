#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

// Diesel ORM
/*
#[macro_use]
extern crate diesel;
extern crate chrono;

pub mod schema;
pub mod models;

use diesel::prelude::*;
*/

// tarpc
#[macro_use]
extern crate tarpc;

use tarpc::util::Never;

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
