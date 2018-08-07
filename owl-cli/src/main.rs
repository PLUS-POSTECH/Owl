#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate clap;
extern crate dotenv;
extern crate env_logger;
extern crate owl_rpc;
extern crate tarpc;
extern crate tokio;
extern crate tokio_core;

use std::net::ToSocketAddrs;

use self::team::{team_command, team_match};
use clap::{App, AppSettings, Arg, SubCommand};
use dotenv::dotenv;
use owl_rpc::model::service::provider::{
    ServiceProviderData, ServiceProviderListParams, ServiceProviderUpdateParams,
};
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
            Arg::from_usage("-c, --config [toml] 'custom config file location'")
                .default_value("config.toml"),
        )
        /*
        if let Some(matches) = matches.subcommand_matches("provider") {
            if let Some(matches) = matches.subcommand_matches("list") {
                let options = client::Options::default().handle(reactor.handle());
                let params = ServiceProviderListParams {
                    show_all: matches.is_present("all"),
                    filter_teams: matches.values_of("filter-team").unwrap().map(|x| x.to_string()).collect(),
                    filter_service_variants: matches.values_of("filter-service-variant").unwrap().map(|x| x.to_string()).collect(),
                };
                let result = reactor.run(FutureClient::connect("localhost:5959".to_socket_addrs().unwrap().next().unwrap(), options)
                    .then(move |client| {
                        client.unwrap().list_service_provider("".to_string(), params)
                    })
                );
            }
            if let Some(matches) = matches.subcommand_matches("update") {

            }
        }
    }
    */

        .subcommands(vec![team_command(), service_command()])
        .get_matches();

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
