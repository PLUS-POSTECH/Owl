#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate clap;
extern crate dotenv;
extern crate env_logger;
extern crate owl_rpc;
extern crate tarpc;

use self::team::{team_command, team_match};
use clap::{App, AppSettings, Arg};
use dotenv::dotenv;
use owl_rpc::SyncClient;
use service::{service_command, service_match};
use tarpc::sync::client::{self, ClientExt};
use tarpc::util::FirstSocketAddr;

mod error;
mod service;
mod team;

pub struct SharedParam {
    pub client: SyncClient,
    pub token: String,
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let matches = App::new("Owl CLI")
        .version("0.1")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("TOML")
                .help("Sets a custom config file (default is \"config.toml\"")
                .takes_value(true),
        )
        .subcommand(team_command())
        .subcommand(service_command())
        .get_matches();

    let _config = matches.value_of("config").unwrap_or("config.toml");

    // TODO: read connection string from config
    let client = match SyncClient::connect(
        "localhost:5959".first_socket_addr(),
        client::Options::default(),
    ) {
        Ok(client) => client,
        Err(_) => {
            error!("Connection failed");
            return;
        },
    };

    let shared_param = SharedParam {
        client,
        // TODO: read token value from config
        token: "token".to_string(),
    };

    let result = match matches.subcommand() {
        ("team", Some(matches)) => team_match(matches, shared_param),
        ("service", Some(matches)) => service_match(matches, shared_param),
        _ => Err(error::Error::InvalidSubcommand),
    };

    match result {
        Ok(str) => println!("{}", str),
        Err(e) => error!("{}", e.to_string()),
    };
}
