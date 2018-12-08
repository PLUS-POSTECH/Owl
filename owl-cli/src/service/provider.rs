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
                .about("list available service providers")
                .args(&[
                    Arg::from_usage("-a, --all 'shows disabled service too'"),
                    Arg::from_usage("-T, --filter-team... [team_name] 'filters providers by team'"),
                    Arg::from_usage("-V, --filter-service-variant... [service_variant_name] 'filters providers by service variant'"),
                ]),
            SubCommand::with_name("update")
                .about("update service provider information (admin)")
                .args(&[
                    Arg::from_usage("-T, --team <team_name> 'name of the team providing service'"),
                    Arg::from_usage("-V, --service-variant <service_variant_name> 'name of the service variant being provided'"),
                    Arg::from_usage("-S, --connection-string <connection_string> 'URI to use when connecting to service'"),
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
                    filter_teams: if matches.is_present("filter-team") {
                        matches
                            .values_of("filter-team")
                            .unwrap()
                            .map(ToString::to_string)
                            .collect()
                    } else {
                        Vec::new()
                    },
                    filter_service_variants: if matches.is_present("filter-service-variant") {
                        matches
                            .values_of("filter-service-variant")
                            .unwrap()
                            .map(ToString::to_string)
                            .collect()
                    } else {
                        Vec::new()
                    },
                },
            )?;

            if service_providers.is_empty() {
                Ok("No service provider registered".to_string())
            } else {
                Ok(service_providers
                    .into_iter()
                    .map(|service_provider| {
                        format!(
                            "- {:20} | {} | {}",
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

            Ok("Provider successfully updated".to_string())
        },

        _ => Err(Error::InvalidSubcommand),
    }
}
