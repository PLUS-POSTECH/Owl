#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate tarpc;

extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tokio;

use tarpc::util::Never;

pub mod db;
pub mod error;

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
