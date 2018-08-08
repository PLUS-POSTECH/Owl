use chrono::{DateTime, Utc};
use db::models::*;
use db::schema::*;
use db::DbPool;
use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::PgConnection;
use digest::Input;
use error::Error;
use owl_rpc::model::service::variant::*;
use owl_rpc::model::FileEntry;
use sha3::{Digest, Sha3_256};

pub fn list_service_variant(
    db_pool: DbPool,
    params: ServiceVariantListParams,
) -> Result<Vec<ServiceVariantData>, Error> {
    let con: &PgConnection = &*db_pool.get()?;
    let show_all = params.show_all;
    let filter_teams = params.filter_teams;

    let mut query = service_variants::table
        .inner_join(teams::table)
        .inner_join(services::table)
        .into_boxed();

    if !filter_teams.is_empty() {
        query = query.filter(teams::name.eq_any(filter_teams));
    }

    if show_all {
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

pub fn edit_service_variant(
    db_pool: DbPool,
    params: ServiceVariantEditParams,
) -> Result<(), Error> {
    let con: &PgConnection = &*db_pool.get()?;

    match params {
        ServiceVariantEditParams::Add {
            service_name: ref param_service_name,
            publisher_name: ref param_publisher_name,
            file_entries: ref param_file_entries,
        } => con.transaction::<(), Error, _>(|| {
            let service = services::table
                .filter(services::name.eq(param_service_name))
                .first::<Service>(con)?;

            let publisher = teams::table
                .filter(teams::name.eq(param_publisher_name))
                .first::<Team>(con)?;

            let mut hasher = Sha3_256::default();
            let first_file_content = &param_file_entries[0].data;

            hasher.process(first_file_content);
            let out = hasher.result();
            let mut hash_print = format!("{:x}", out);
            hash_print.truncate(8);

            let inserted_variant = diesel::insert_into(service_variants::table)
                .values(ServiceVariantInsertable {
                    service_id: service.id,
                    name: format!("{}-{}", service.name, hash_print),
                    publisher_id: publisher.id,
                })
                .get_result::<ServiceVariant>(con)?;

            for file_entry in param_file_entries {
                diesel::insert_into(service_variant_attachments::table)
                    .values(ServiceVariantAttachmentInsertable {
                        service_variant_id: inserted_variant.id,
                        name: file_entry.name.clone(),
                        data: file_entry.data.clone(),
                    })
                    .execute(con)?;
            }
            Ok(())
        }),
        ServiceVariantEditParams::Delete {
            name: ref param_name,
        } => {
            let rows = diesel::delete(
                service_variants::table.filter(service_variants::name.eq(param_name)),
            ).execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Service {} not found", param_name)))
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
    db_pool: DbPool,
    params: ServiceVariantDownloadParams,
) -> Result<ServiceVariantAttachmentData, Error> {
    let con: &PgConnection = &*db_pool.get()?;
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