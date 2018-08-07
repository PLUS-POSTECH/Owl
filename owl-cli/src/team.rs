use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error::Error;
use owl_rpc::model::team::*;
use SharedParam;

pub fn team_command() -> App<'static, 'static> {
    SubCommand::with_name("team")
        .about("CTF team management")
        .setting(AppSettings::SubcommandRequired)
        .subcommands(vec![
            SubCommand::with_name("add")
                .about("add team (admin)")
                .args(&[
                    Arg::from_usage("<name> 'team name'"),
                    Arg::from_usage("[description] 'team description'"),
                ]),
            SubCommand::with_name("delete")
                .about("delete team (admin)")
                .args(&[Arg::from_usage("<name> 'team name'")]),
            SubCommand::with_name("update")
                .about("update team (admin)")
                .args(&[
                    Arg::from_usage("<name> 'team name'"),
                    Arg::from_usage("<description> 'team description'"),
                ]),
            SubCommand::with_name("list").about("list teams"),
        ])
}

pub fn team_match(matches: &ArgMatches, shared_param: SharedParam) -> Result<String, Error> {
    match matches.subcommand() {
        ("add", Some(matches)) => {
            shared_param.client.edit_team(
                shared_param.token,
                TeamEditParams::Add {
                    name: matches.value_of("name").unwrap().to_string(),
                    description: matches.value_of("description").unwrap_or("").to_string(),
                },
            )?;

            Ok("Team successfully added".to_string())
        },

        ("delete", Some(matches)) => {
            shared_param.client.edit_team(
                shared_param.token,
                TeamEditParams::Delete {
                    name: matches.value_of("name").unwrap().to_string(),
                },
            )?;

            Ok("Team successfully deleted".to_string())
        },

        ("update", Some(matches)) => {
            shared_param.client.edit_team(
                shared_param.token,
                TeamEditParams::Update {
                    name: matches.value_of("name").unwrap().to_string(),
                    description: matches.value_of("description").map(ToString::to_string),
                },
            )?;

            Ok("Team successfully updated".to_string())
        },

        ("list", Some(matches)) => {
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
        },

        _ => Err(Error::InvalidSubcommand),
    }
}
