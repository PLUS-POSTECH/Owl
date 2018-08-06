extern crate clap;
extern crate owl_rpc;
extern crate tarpc;

use self::team::team_command;
use clap::{App, Arg, SubCommand};
use owl_rpc::FutureClient;
use tarpc::future::client::{self, ClientExt};

mod team;

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
        .subcommand(team_command())
        .get_matches();

    let config = matches.value_of("config").unwrap_or("config.toml");
    println!("Value for config: {}", config);

    if let Some(matches) = matches.subcommand_matches("team") {}
}
