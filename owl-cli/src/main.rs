extern crate clap;
extern crate owl_rpc;
extern crate tarpc;

use clap::{App, Arg, SubCommand};
use owl_rpc::FutureClient;
use tarpc::future::client::{self, ClientExt};

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
        .subcommand(SubCommand::with_name("team").about("CTF team management"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("config.toml");
    println!("Value for config: {}", config);

    if let Some(_matches) = matches.subcommand_matches("team") {}
}
