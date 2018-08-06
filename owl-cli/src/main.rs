extern crate clap;
extern crate futures;
extern crate owl_rpc;
extern crate tarpc;
extern crate tokio_core;

use self::team::{team_command, team_match};
use clap::{App, AppSettings, Arg};
use futures::Future;
use owl_rpc::FutureClient;
use tarpc::future::client::{self, ClientExt};
use tarpc::util::{FirstSocketAddr, Message};
use tokio_core::reactor;

mod team;

fn main() {
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
        .get_matches();

    let config = matches.value_of("config").unwrap_or("config.toml");
    println!("Value for config: {}", config);

    let mut reactor = reactor::Core::new().expect("Failed to initialize tokio reactor");
    let options = client::Options::default().handle(reactor.handle());

    let main_future = FutureClient::connect("localhost:5959".first_socket_addr(), options)
        .map_err(tarpc::Error::from)
        // TODO: Fix tarpc error handling
        .map_err(|_e: tarpc::Error<FutureClient>| Message("Some error happened".to_string()))
        .and_then(|client| -> Result<String, Message> {
            if let Some(matches) = matches.subcommand_matches("team") {
                team_match(matches, &client)
            } else {
                Ok("It was not a team related command...".to_string())
            }
        })
        .map(|resp| println!("{}", resp));

    if let Err(err) = reactor.run(main_future) {
        eprintln!("Error: {:?}", err)
    };
}
