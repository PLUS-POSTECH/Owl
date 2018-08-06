extern crate clap;
extern crate futures;
extern crate owl_rpc;
extern crate tarpc;
extern crate tokio;
extern crate tokio_core;

use std::net::ToSocketAddrs;

use clap::{App, Arg, SubCommand};
use futures::Future;
use owl_rpc::FutureClient;
use owl_rpc::model::service::provider::{ServiceProviderData, ServiceProviderListParams, ServiceProviderUpdateParams};
use tarpc::future::client::{self, ClientExt};
use tokio::runtime::Runtime;
use tokio_core::reactor;

fn main() {
    let matches = App::new("Owl CLI")
        .version("0.1")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("TOML")
                .help("Sets a custom config file (default is \"config.toml\"")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("team").about("CTF team management"),
            SubCommand::with_name("service").about("CTF service management")
                .subcommands(vec![
                    SubCommand::with_name("provider").about("CTF service provider management")
                        .subcommands(vec![
                            SubCommand::with_name("list")
                                .about("List available service providers")
                                .args(&[
                                    Arg::with_name("all")
                                        .short("a")
                                        .long("all")
                                        .help("Shows disabled service also"),
                                    Arg::with_name("filter-team")
                                        .short("t")
                                        .long("filter-team")
                                        .value_name("TEAM_NAME")
                                        .help("Filters providers by team")
                                        .multiple(true)
                                        .takes_value(true),
                                    Arg::with_name("filter-service-variant")
                                        .short("v")
                                        .long("filter-service-variant")
                                        .value_name("SERVICE_VARIANT_NAME")
                                        .help("Filters providers by service variant")
                                        .multiple(true)
                                        .takes_value(true),
                                ]),
                            SubCommand::with_name("update")
                                .about("Update service provider information")
                                .args(&[
                                    Arg::with_name("team")
                                        .short("t")
                                        .long("team")
                                        .value_name("TEAM_NAME")
                                        .help("Team information")
                                        .required(true)
                                        .takes_value(true),
                                    Arg::with_name("service-variant")
                                        .short("v")
                                        .long("service-variant")
                                        .value_name("SERVICE_VARIANT_NAME")
                                        .help("Service variant information")
                                        .required(true)
                                        .takes_value(true),
                                    Arg::with_name("connection-string")
                                        .short("c")
                                        .long("connection-string")
                                        .value_name("CONNECTION_STRING")
                                        .help("Access URI for service provider")
                                        .takes_value(true),
                                ]),
                        ])
                ])
        ])

        .get_matches();

    let mut reactor = reactor::Core::new().expect("Creating reactor failed");

    let config = matches.value_of("config").unwrap_or("config.toml");
    println!("Value for config: {}", config);

    if let Some(_matches) = matches.subcommand_matches("team") {}
    if let Some(matches) = matches.subcommand_matches("service") {
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

}
