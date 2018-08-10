use chrono::{DateTime, Utc};
use db::models::*;
use db::schema::*;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use digest::Input;
use error::Error;
use owl_rpc::model::service::variant::*;
use owl_rpc::model::FileEntry;
use sha3::{Digest, Sha3_256};
use DaemonResource;

pub fn list_service_variant(
    resource: &DaemonResource,
    params: ServiceVariantListParams,
) -> Result<Vec<ServiceVariantData>, Error> {
    let con: &PgConnection = &*resource.db_pool.get()?;
    let show_all = params.show_all;
    let filter_teams = params.filter_teams;

    let mut query = service_variants::table
        .inner_join(teams::table)
        .inner_join(services::table)
        .into_boxed();

    if !filter_teams.is_empty() {
        query = query.filter(teams::name.eq_any(filter_teams));
    }

    if !show_all {
        query = query.filter(services::enabled.eq(true));
    }

    let result = query
        .select((
            service_variants::name,
            services::name,
            service_variants::publisher_id,
            service_variants::published_time,
        ))
        .load(con)?
        .into_iter()
        .map(|x: (String, String, i32, DateTime<Utc>)| {
            Ok(ServiceVariantData {
                name: x.0,
                service_name: x.1,
                publisher_name: teams::table.find(x.2).first::<Team>(con)?.name,
                published_time: x.3,
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(result)
}

pub fn add_service_variant(
    resource: &DaemonResource,
    params: ServiceVariantAddParams,
) -> Result<String, Error> {
    let con: &PgConnection = &*resource.db_pool.get()?;

    con.transaction::<String, Error, _>(|| {
        let service = services::table
            .filter(services::name.eq(&params.service_name))
            .first::<Service>(con)?;

        let publisher = teams::table
            .filter(teams::name.eq(&params.publisher_name))
            .first::<Team>(con)?;

        let service_variant_hash = {
            let mut hasher = Sha3_256::default();

            for file_entry in &params.file_entries {
                hasher.process(&file_entry.data);
            }
            hasher.result()
        };

        let mut hash_print = format!("{:x}", service_variant_hash);
        hash_print.truncate(8);

        let inserted_variant = diesel::insert_into(service_variants::table)
            .values(ServiceVariantInsertable {
                service_id: service.id,
                name: format!("{}-{}", service.name, hash_print),
                publisher_id: publisher.id,
            })
            .get_result::<ServiceVariant>(con)?;

        for file_entry in params.file_entries {
            diesel::insert_into(service_variant_attachments::table)
                .values(ServiceVariantAttachmentInsertable {
                    service_variant_id: inserted_variant.id,
                    name: file_entry.name,
                    data: file_entry.data,
                })
                .execute(con)?;
        }
        Ok(inserted_variant.name)
    })
}

pub fn edit_service_variant(
    resource: &DaemonResource,
    params: ServiceVariantEditParams,
) -> Result<(), Error> {
    let con: &PgConnection = &*resource.db_pool.get()?;

    match params {
        ServiceVariantEditParams::Delete { name: param_name } => {
            let rows = diesel::delete(
                service_variants::table.filter(service_variants::name.eq(&param_name)),
            ).execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Service {} not found", &param_name)))
            } else {
                Ok(())
            }
        },
        ServiceVariantEditParams::Update {
            name: param_name,
            service_name: param_service_name,
            publisher_name: param_publisher_name,
            sla_pass: param_sla_pass,
        } => {
            let service_id = match param_service_name {
                Some(name) => Some(
                    services::table
                        .filter(services::name.eq(name))
                        .first::<Service>(con)?
                        .id,
                ),
                None => None,
            };

            let publisher_id = match param_publisher_name {
                Some(name) => Some(
                    teams::table
                        .filter(teams::name.eq(name))
                        .first::<Team>(con)?
                        .id,
                ),
                None => None,
            };

            // TODO: Change service variant name on service change

            let rows = diesel::update(
                service_variants::table.filter(service_variants::name.eq(&param_name)),
            ).set(ServiceVariantChangeset {
                service_id,
                name: None,
                sla_pass: param_sla_pass,
                publisher_id,
            })
                .execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Service {} not found", param_name)))
            } else {
                Ok(())
            }
        },
    }
}

pub fn download_service_variant(
    resource: &DaemonResource,
    params: ServiceVariantDownloadParams,
) -> Result<ServiceVariantAttachmentData, Error> {
    let con: &PgConnection = &*resource.db_pool.get()?;
    let service_variant_name = params.name;

    let service_variant = service_variants::table
        .filter(service_variants::name.eq(service_variant_name))
        .first::<ServiceVariant>(con)?;

    let service_variant_attachments = service_variant_attachments::table
        .filter(service_variant_attachments::service_variant_id.eq(service_variant.id))
        .load::<ServiceVariantAttachment>(con)?;

    let result = ServiceVariantAttachmentData {
        file_entries: service_variant_attachments
            .into_iter()
            .map(|x| FileEntry {
                name: x.name,
                data: x.data,
            })
            .collect(),
    };
    Ok(result)
}
