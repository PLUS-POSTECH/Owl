#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Owl CLI")
        .version("0.1")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("TOML")
            .help("Sets a custom config file (default is \"config.toml\"")
            .takes_value(true))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("config.toml");
    println!("Value for config: {}", config);
}
