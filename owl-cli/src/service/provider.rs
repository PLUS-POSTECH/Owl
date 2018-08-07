use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error::Error;
use owl_rpc::model::service::provider::*;
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
                    Arg::from_usage("-s, --connection-string <connection_string> 'URI to use when connecting to service'"),
                ]),
        ])
}

pub fn service_provider_match(
    matches: &ArgMatches,
    shared_param: SharedParam,
) -> Result<String, Error> {
    match matches.subcommand() {
        ("list", Some(matches)) => {
            let service_providers = shared_param.client.list_service_provider(
                shared_param.token,
                ServiceProviderListParams {
                    show_all: matches.is_present("all"),
                    filter_teams: matches
                        .values_of("filter-team")
                        .unwrap()
                        .map(|x| x.to_string())
                        .collect(),
                    filter_service_variants: matches
                        .values_of("filter-service_variant")
                        .unwrap()
                        .map(|x| x.to_string())
                        .collect(),
                },
            )?;

            if service_providers.is_empty() {
                Ok("No service provider registered".to_string())
            } else {
                Ok(service_providers
                    .into_iter()
                    .map(|service_provider| {
                        format!(
                            "- {:10} | {:10} | {}",
                            service_provider.team_name,
                            service_provider.service_variant_name,
                            service_provider.connection_string
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
        },

        ("update", Some(matches)) => {
            shared_param.client.update_service_provider(
                shared_param.token,
                ServiceProviderUpdateParams {
                    team_name: matches.value_of("team").unwrap().to_string(),
                    service_variant_name: matches.value_of("service-variant").unwrap().to_string(),
                    connection_string: matches.value_of("connection-string").unwrap().to_string(),
                },
            )?;

            Ok("Service successfully deleted".to_string())
        },

        _ => Err(Error::InvalidSubcommand),
    }
}
