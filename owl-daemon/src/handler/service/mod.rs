use db::models::{Service, ServiceChangeset};
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::service::*;
use DaemonResource;

pub fn edit_service(resource: &DaemonResource, params: ServiceEditParams) -> Result<(), Error> {
    use db::schema::services::dsl::*;

    let con: &PgConnection = &*resource.db_pool.get()?;

    match params {
        ServiceEditParams::Add {
            name: ref param_name,
            description: ref param_description,
        } => {
            diesel::insert_into(services)
                .values((
                    name.eq(param_name),
                    description.eq(param_description),
                    enabled.eq(true),
                ))
                .execute(con)?;

            Ok(())
        },
        ServiceEditParams::Delete {
            name: ref param_name,
        } => {
            let rows = diesel::delete(services.filter(name.eq(param_name))).execute(con)?;

            if rows == 0 {
                Err(Error::Message(format!("Service {} not found", param_name)))
            } else {
                Ok(())
            }
        },
        ServiceEditParams::Update {
            name: ref param_name,
            description: ref param_description,
            enabled: param_enabled,
        } => {
            let rows = diesel::update(services.filter(name.eq(param_name)))
                .set(ServiceChangeset {
                    name: None,
                    description: param_description.clone(),
                    enabled: param_enabled,
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

pub fn list_service(
    resource: &DaemonResource,
    params: ServiceListParams,
) -> Result<Vec<ServiceData>, Error> {
    use db::schema::services::dsl::*;

    let con: &PgConnection = &*resource.db_pool.get()?;

    let fetch = if params.show_all {
        services.load::<Service>(con)?
    } else {
        services.filter(enabled.eq(true)).load::<Service>(con)?
    };

    Ok(fetch
        .into_iter()
        .map(|service| ServiceData {
            name: if service.enabled {
                service.name
            } else {
                format!("({})", service.name)
            },
            description: service.description,
            published_time: service.published_time,
        })
        .collect())
}
