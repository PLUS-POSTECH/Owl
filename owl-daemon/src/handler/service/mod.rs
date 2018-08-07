use db::models::{Service, ServiceChangeset};
use db::DbPool;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::service::*;

pub fn edit_service(db_pool: DbPool, params: ServiceEditParams) -> Result<(), Error> {
    use db::schema::services::dsl::*;

    let con: &PgConnection = &*db_pool.get()?;

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

pub fn list_service(db_pool: DbPool) -> Result<Vec<ServiceData>, Error> {
    use db::schema::services::dsl::*;

    let con: &PgConnection = &*db_pool.get()?;

    let fetch = services.load::<Service>(con)?;
    Ok(fetch
        .into_iter()
        .map(|service| ServiceData {
            name: service.name,
            description: service.description,
            published_time: service.published_time,
        })
        .collect())
}