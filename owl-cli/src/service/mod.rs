use self::provider::{service_provider_command, service_provider_match};
use self::variant::{service_variant_command, service_variant_match};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error::Error;
use owl_rpc::model::service::*;
use SharedParam;

pub mod provider;
pub mod variant;

pub fn service_command() -> App<'static, 'static> {
    SubCommand::with_name("service")
        .about("CTF service management")
        .setting(AppSettings::SubcommandRequired)
        .subcommands(vec![
            SubCommand::with_name("add")
                .about("add service (admin)")
                .args(&[
                    Arg::from_usage("<name> 'service name'"),
                    Arg::from_usage("[description] 'service description'"),
                ]),
            SubCommand::with_name("delete")
                .about("delete service (admin)")
                .args(&[Arg::from_usage("<name> 'service name'")]),
            SubCommand::with_name("enable")
                .about("enable service (admin)")
                .args(&[Arg::from_usage("<name> 'service name'")]),
            SubCommand::with_name("disable")
                .about("disable service (admin)")
                .args(&[Arg::from_usage("<name> 'service name'")]),
            SubCommand::with_name("update")
                .about("update service description (admin)")
                .args(&[
                    Arg::from_usage("<name> 'service name'"),
                    Arg::from_usage("<description> 'service description'"),
                ]),
            SubCommand::with_name("list")
                .about("list running services")
                .args(&[Arg::from_usage("-a, --all 'include disabled services'")]),
            service_provider_command(),
            service_variant_command(),
        ])
}

pub fn service_match(matches: &ArgMatches, shared_param: SharedParam) -> Result<String, Error> {
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

        ("provider", Some(matches)) => service_provider_match(matches, shared_param),

        ("variant", Some(matches)) => service_variant_match(matches, shared_param),

        _ => Err(Error::InvalidSubcommand),
    }
}
