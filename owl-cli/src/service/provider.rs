use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error::Error;
use owl_rpc::model::service::*;
use SharedParam;

pub fn service_provider_command() -> App<'static, 'static> {
    SubCommand::with_name("provider")
        .about("CTF service provider management")
        .setting(AppSettings::SubcommandRequired)
        .subcommands(vec![
            SubCommand::with_name("list")
                .about("List available service providers")
                .args(&[
                    Arg::from_usage("-a, --all 'Shows disabled service also'"),
                    Arg::from_usage("-t, --filter-team [team_name] 'Filters providers by team'")
                        .multiple(true),
                    Arg::from_usage("-v, --filter-service-variant [service_variant_name] 'Filters providers by service variant'")
                        .multiple(true),
                ]),
            SubCommand::with_name("update")
                .about("Update service provider information")
                .args(&[
                    Arg::from_usage("-t, --team <team_name> 'Name of the team providing service'"),
                    Arg::from_usage("-v, --service-variant <service_variant_name> 'Name of the service variant being provided'"),
                    Arg::from_usage("-s, --connection-string [connection_string] 'URI to use when connecting to service'"),
                ]),
        ])
}

pub fn service_provider_match(
    matches: &ArgMatches,
    shared_param: SharedParam,
) -> Result<String, Error> {
    match matches.subcommand() {
        ("add", Some(matches)) => {
            shared_param.client.edit_service(
                shared_param.token,
                ServiceEditParams::Add {
                    name: matches.value_of("name").unwrap().to_string(),
                    description: matches.value_of("description").unwrap_or("").to_string(),
                },
            )?;

            Ok("Service successfully added".to_string())
        },

        ("delete", Some(matches)) => {
            shared_param.client.edit_service(
                shared_param.token,
                ServiceEditParams::Delete {
                    name: matches.value_of("name").unwrap().to_string(),
                },
            )?;

            Ok("Service successfully deleted".to_string())
        },

        ("enable", Some(matches)) => {
            shared_param.client.edit_service(
                shared_param.token,
                ServiceEditParams::Update {
                    name: matches.value_of("name").unwrap().to_string(),
                    description: None,
                    enabled: Some(true),
                },
            )?;

            Ok("Service successfully updated".to_string())
        },

        ("disable", Some(matches)) => {
            shared_param.client.edit_service(
                shared_param.token,
                ServiceEditParams::Update {
                    name: matches.value_of("name").unwrap().to_string(),
                    description: None,
                    enabled: Some(false),
                },
            )?;

            Ok("Service successfully updated".to_string())
        },

        ("update", Some(matches)) => {
            shared_param.client.edit_service(
                shared_param.token,
                ServiceEditParams::Update {
                    name: matches.value_of("name").unwrap().to_string(),
                    description: matches.value_of("description").map(ToString::to_string),
                    enabled: None,
                },
            )?;

            Ok("Service successfully updated".to_string())
        },

        ("list", Some(matches)) => {
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
        },

        _ => Err(Error::InvalidSubcommand),
    }
}
