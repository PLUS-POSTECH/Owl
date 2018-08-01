extern crate clap;
extern crate owl_daemon;
extern crate tarpc;

use clap::{App, Arg, SubCommand};
use owl_daemon::SyncClient;
use tarpc::sync::{client, client::ClientExt};

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
        .subcommand(SubCommand::with_name("test").about("test basic connection"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("config.toml");
    println!("Value for config: {}", config);

    if let Some(_matches) = matches.subcommand_matches("test") {
        let client = SyncClient::connect("localhost:5959", client::Options::default()).unwrap();
        println!("{}", client.hello("Mom".to_string()).unwrap());
    }
}
