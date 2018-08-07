use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error::Error;
use owl_rpc::model::service::*;
use SharedParam;

pub fn service_command() -> App<'static, 'static> {
    SubCommand::with_name("service")
        .about("CTF service management")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("add")
                .about("add service (admin)")
                .args(&[
                    Arg::from_usage("<name> 'service name'"),
                    Arg::from_usage("[description] 'service description'"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("delete service (admin)")
                .args(&[Arg::from_usage("<name> 'service name'")]),
        )
        .subcommand(
            SubCommand::with_name("enable")
                .about("enable service (admin)")
                .args(&[Arg::from_usage("<name> 'service name'")]),
        )
        .subcommand(
            SubCommand::with_name("disable")
                .about("disable service (admin)")
                .args(&[Arg::from_usage("<name> 'service name'")]),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("update service (admin)")
                .args(&[
                    Arg::from_usage("<name> 'service name'"),
                    Arg::from_usage("<description> 'service description'"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("list services")
                .args(&[Arg::from_usage("-a, --all 'include disabled services'")]),
        )
}

pub fn service_match(matches: &ArgMatches, shared_param: SharedParam) -> Result<String, Error> {
    if let Some(matches) = matches.subcommand_matches("add") {
        shared_param.client.edit_service(
            shared_param.token,
            ServiceEditParams::Add {
                name: matches.value_of("name").unwrap().to_string(),
                description: matches.value_of("description").unwrap_or("").to_string(),
            },
        )?;

        Ok("Service successfully added".to_string())
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        shared_param.client.edit_service(
            shared_param.token,
            ServiceEditParams::Delete {
                name: matches.value_of("name").unwrap().to_string(),
            },
        )?;

        Ok("Service successfully deleted".to_string())
    } else if let Some(matches) = matches.subcommand_matches("enable") {
        shared_param.client.edit_service(
            shared_param.token,
            ServiceEditParams::Update {
                name: matches.value_of("name").unwrap().to_string(),
                description: None,
                enabled: Some(true),
            },
        )?;

        Ok("Service successfully updated".to_string())
    } else if let Some(matches) = matches.subcommand_matches("disable") {
        shared_param.client.edit_service(
            shared_param.token,
            ServiceEditParams::Update {
                name: matches.value_of("name").unwrap().to_string(),
                description: None,
                enabled: Some(false),
            },
        )?;

        Ok("Service successfully updated".to_string())
    } else if let Some(matches) = matches.subcommand_matches("update") {
        shared_param.client.edit_service(
            shared_param.token,
            ServiceEditParams::Update {
                name: matches.value_of("name").unwrap().to_string(),
                description: matches.value_of("description").map(ToString::to_string),
                enabled: None,
            },
        )?;

        Ok("Service successfully updated".to_string())
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let services = shared_param.client.list_service(
            shared_param.token,
            ServiceListParams {
                show_all: matches.is_present("all"),
            },
        )?;

        if services.is_empty() {
            Ok("No service registered".to_string())
        } else {
            Ok(services
                .into_iter()
                .map(|service| format!("- {:10} | {}", service.name, service.description))
                .collect::<Vec<_>>()
                .join("\n"))
        }
    } else {
        Err(Error::InvalidSubcommand)
    }
}
