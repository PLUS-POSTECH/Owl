#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate dotenv;
extern crate env_logger;
extern crate owl_rpc;
extern crate tarpc;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

use self::exploit::{exploit_command, exploit_match};
use self::service::{service_command, service_match};
use self::team::{team_command, team_match};
use clap::{App, AppSettings, Arg};
use dotenv::dotenv;
use error::Error;
use owl_rpc::SyncClient;
use tarpc::sync::client::{self, ClientExt};
use tarpc::util::FirstSocketAddr;

mod error;
mod exploit;
mod service;
mod team;

#[derive(Deserialize)]
struct Config {
    client: Client,
}

#[derive(Deserialize)]
struct Client {
    connection: String,
    token: String,
}

pub struct SharedParam {
    pub client: SyncClient,
    pub token: String,
}

pub fn read_file_contents(file_name: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(file_name)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main_wrap() -> Result<String, Error> {
    dotenv().ok();
    env_logger::init();

    let matches = App::new("Owl CLI")
        .version("0.1")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::from_usage("-c, --config [toml] 'custom config file location'")
                .default_value("config.toml"),
        )
        .subcommands(vec![team_command(), service_command(), exploit_command()])
        .get_matches();

    let config: Config =
        toml::from_slice(&read_file_contents(matches.value_of("config").unwrap())?)?;

    let client = SyncClient::connect(
        config.client.connection.try_first_socket_addr()?,
        client::Options::default(),
    )?;

    let shared_param = SharedParam {
        client,
        token: config.client.token,
    };

    match matches.subcommand() {
        ("team", Some(matches)) => team_match(matches, shared_param),
        ("service", Some(matches)) => service_match(matches, shared_param),
        ("exploit", Some(matches)) => exploit_match(matches, shared_param),
        _ => Err(error::Error::InvalidSubcommand),
    }
}

fn main() {
    match main_wrap() {
        Ok(s) => println!("{}", s),
        Err(e) => error!("{}", e.to_string()),
    };
}
