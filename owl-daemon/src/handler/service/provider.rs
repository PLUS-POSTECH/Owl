use chrono::{DateTime, Utc};
use db::models::*;
use db::schema::*;
use db::DbPool;
use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::service::provider::{
    ServiceProviderData, ServiceProviderListParams, ServiceProviderUpdateParams,
};

pub fn list_service_provider(
    db_pool: DbPool,
    params: ServiceProviderListParams,
) -> Result<Vec<ServiceProviderData>, Error> {
    let conn: &PgConnection = &*db_pool.get()?;
    let show_all = params.show_all;
    let filter_teams = params.filter_teams;
    let filter_service_variants = params.filter_service_variants;

    let query = service_providers::table
        .inner_join(teams::table)
        .inner_join(service_variants::table.inner_join(services::table))
        .filter(teams::name.eq_any(filter_teams))
        .filter(service_variants::name.eq_any(filter_service_variants));

    let result = if show_all {
        query
            .filter(services::enabled.eq(true))
            .select((
                teams::name,
                service_variants::name,
                service_providers::connection_string,
                service_providers::published_time,
            ))
            .load(conn)
    } else {
        query
            .select((
                teams::name,
                service_variants::name,
                service_providers::connection_string,
                service_providers::published_time,
            ))
            .load(conn)
    }?.into_iter()
        .map(
            |x: (String, String, String, DateTime<Utc>)| ServiceProviderData {
                team_name: x.0,
                service_variant_name: x.1,
                connection_string: x.2,
                published_time: x.3,
            },
        )
        .collect();
    Ok(result)
}

pub fn update_service_provider(
    db_pool: DbPool,
    params: ServiceProviderUpdateParams,
) -> Result<(), Error> {
    let conn: &PgConnection = &*db_pool.get()?;
    let team_name = params.team_name;
    let service_variant_name = params.service_variant_name;
    let connection_string = params.connection_string;

    conn.transaction::<(), Error, _>(|| {
        let service_variant = service_variants::table
            .filter(service_variants::name.eq(service_variant_name))
            .first::<ServiceVariant>(conn)?;

        let team = teams::table
            .filter(teams::name.eq(team_name))
            .first::<Team>(conn)?;

        let result = service_providers::table
            .inner_join(service_variants::table)
            .filter(service_providers::team_id.eq(team.id))
            .filter(service_variants::service_id.eq(service_variant.service_id))
            .order_by(service_providers::published_time.desc())
            .first::<(ServiceProvider, ServiceVariant)>(conn);

        if let Ok((record, _)) = result {
            let target = service_providers::table.find(record.id);

            if (record.team_id, record.service_variant_id) == (team.id, service_variant.id) {
                diesel::update(target)
                    .set(ServiceProviderChangeset {
                        connection_string: Some(connection_string),
                    })
                    .execute(conn)?;
            }
        } else {
            diesel::insert_into(service_providers::table)
                .values(ServiceProviderInsertable {
                    team_id: team.id,
                    service_variant_id: service_variant.id,
                    connection_string,
                })
                .execute(conn)?;
        }

        Ok(())
    })
}
