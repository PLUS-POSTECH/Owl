use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error::Error;
use owl_rpc::model::team::*;
use SharedParam;

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

pub fn team_match(matches: &ArgMatches, shared_param: SharedParam) -> Result<String, Error> {
    if let Some(matches) = matches.subcommand_matches("add") {
        let teams = shared_param.client.edit_team(
            shared_param.token,
            TeamEditParams::Add(TeamAddParams {
                name: matches.value_of("NAME").unwrap().to_string(),
                description: matches.value_of("DESCRIPTION").unwrap_or("").to_string(),
            }),
        )?;

        Ok("Successfully added a team".to_string())
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        Err(Error::NotImplemented)
    } else if let Some(matches) = matches.subcommand_matches("update") {
        Err(Error::NotImplemented)
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let teams = shared_param.client.list_team(shared_param.token)?;

        if teams.is_empty() {
            Ok("No team registered".to_string())
        } else {
            Ok(teams
                .into_iter()
                .map(|team| format!("- {:10} | {}", team.name, team.description))
                .collect::<Vec<_>>()
                .join("\n"))
        }
    } else {
        Err(Error::InvalidSubcommand)
    }
}
