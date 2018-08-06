use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use owl_rpc::FutureClient;
use tarpc::util::Message;

pub fn team_command() -> App<'static, 'static> {
    SubCommand::with_name("team")
        .about("CTF team management (admin)")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("add").about("add team").args(&[
            Arg::from_usage("<NAME> 'team name'"),
            Arg::from_usage("[DESCRIPTION] 'team description'"),
        ]))
        .subcommand(
            SubCommand::with_name("delete")
                .about("delete team")
                .args(&[Arg::from_usage("<NAME> 'team name'")]),
        )
        .subcommand(SubCommand::with_name("update").about("update team").args(&[
            Arg::from_usage("<NAME> 'team name'"),
            Arg::from_usage("-d, --description [DESCRIPTION] 'team description'"),
        ]))
        .subcommand(SubCommand::with_name("list").about("list team"))
}

pub fn team_match(matches: &ArgMatches, client: &FutureClient) -> Result<String, Message> {
    Ok("It was a team related command!".to_string())
}
