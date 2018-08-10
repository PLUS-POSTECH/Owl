use std::fs::File;
use std::io::{Read, Write};

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error::Error;
use owl_rpc::model::service::variant::*;
use owl_rpc::model::FileEntry;
use SharedParam;

pub fn service_variant_command() -> App<'static, 'static> {
    SubCommand::with_name("variant")
        .about("CTF service variant management")
        .setting(AppSettings::SubcommandRequired)
        .subcommands(vec![
            SubCommand::with_name("add")
                .about("add new service variant (admin)")
                .args(&[
                    Arg::from_usage("<file>... 'binary file(s) representing service variant'"),
                    Arg::from_usage(
                        "-s, --service <service_name> 'classification of the service variant'",
                    ),
                    Arg::from_usage(
                        "-p, --publisher <team_name> 'name of the team who published the variant'",
                    ),
                ]),
            SubCommand::with_name("delete")
                .about("delete specified service variant (admin)")
                .args(&[Arg::from_usage(
                    "<name> 'name of the service variant to delete'",
                )]),
            SubCommand::with_name("update")
                .about("update service variant (admin)")
                .args(&[
                    Arg::from_usage("<name> 'name of the service variant to update'"),
                    Arg::from_usage(
                        "-s, --service [service_name] 'classification of the service variant'",
                    ),
                    Arg::from_usage(
                        "-p, --publisher [team_name] 'name of the team who published the variant'",
                    ),
                    Arg::from_usage(
                        "-V, --sla-pass [sla_pass] 'indicator whether the variant passed SLA check'",
                    ).possible_values(&["true", "false", "none"]),
                ]),
            SubCommand::with_name("list")
                .about("list available service providers")
                .args(&[
                    Arg::from_usage("-a, --all 'shows disabled service also'"),
                    Arg::from_usage("-T, --filter-team... [team_name] 'filters variants by team'"),
                ]),
            SubCommand::with_name("download")
                .about("download service variant")
                .args(&[Arg::from_usage(
                    "<name> 'name of the service variant to download'",
                )]),
        ])
}

pub fn service_variant_match(
    matches: &ArgMatches,
    shared_param: SharedParam,
) -> Result<String, Error> {
    match matches.subcommand() {
        ("add", Some(matches)) => {
            let file_entries = matches
                .values_of("file")
                .unwrap()
                .map(|filename| {
                    let mut file = File::open(filename)?;
                    let mut data = Vec::new();
                    file.read_to_end(&mut data)?;

                    Ok(FileEntry {
                        name: filename.to_string(),
                        data,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;

            shared_param.client.edit_service_variant(
                shared_param.token,
                ServiceVariantEditParams::Add {
                    service_name: matches.value_of("service").unwrap().to_string(),
                    publisher_name: matches.value_of("publisher").unwrap().to_string(),
                    file_entries,
                },
            )?;

            Ok("Variant successfully added".to_string())
        },

        ("delete", Some(matches)) => {
            shared_param.client.edit_service_variant(
                shared_param.token,
                ServiceVariantEditParams::Delete {
                    name: matches.value_of("name").unwrap().to_string(),
                },
            )?;

            Ok("Variant successfully deleted".to_string())
        },

        ("update", Some(matches)) => {
            shared_param.client.edit_service_variant(
                shared_param.token,
                ServiceVariantEditParams::Update {
                    name: matches.value_of("name").unwrap().to_string(),
                    service_name: matches.value_of("service").map(ToString::to_string),
                    publisher_name: matches.value_of("publisher").map(ToString::to_string),
                    sla_pass: match matches.value_of("sla-pass") {
                        Some("true") => Some(Some(true)),
                        Some("false") => Some(Some(false)),
                        Some("none") => Some(None),
                        _ => None,
                    },
                },
            )?;

            Ok("Variant successfully updated".to_string())
        },

        ("list", Some(matches)) => {
            let service_variants = shared_param.client.list_service_variant(
                shared_param.token,
                ServiceVariantListParams {
                    show_all: matches.is_present("all"),
                    filter_teams: matches
                        .values_of("filter-team")
                        .map(|values| -> Vec<_> { values.map(ToString::to_string).collect() })
                        .unwrap_or(Vec::new()),
                },
            )?;

            if service_variants.is_empty() {
                Ok("No service variant registered".to_string())
            } else {
                Ok(service_variants
                    .into_iter()
                    .map(|service_variant| {
                        format!(
                            "- {:10} | {:10} | {:10} | {}",
                            service_variant.name,
                            service_variant.service_name,
                            service_variant.publisher_name,
                            service_variant.published_time
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
        },

        ("download", Some(matches)) => {
            println!("Downloading files...");
            let file_entries = shared_param
                .client
                .download_service_variant(
                    shared_param.token,
                    ServiceVariantDownloadParams {
                        name: matches.value_of("name").unwrap().to_string(),
                    },
                )?
                .file_entries;

            for file_entry in &file_entries {
                let file_name = &file_entry.name;
                let file_contents = &file_entry.data;

                println!("Writing {}...", file_name);
                let mut file = File::create(file_name)?;
                file.write_all(&file_contents[..])?;
            }

            Ok("Variant successfully downloaded".to_string())
        },

        _ => Err(Error::InvalidSubcommand),
    }
}
