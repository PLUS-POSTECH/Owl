use chrono::{DateTime, Utc};
use db::models::*;
use db::schema::*;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::service::provider::*;
use DaemonResource;

pub fn list_service_provider(
    resource: &DaemonResource,
    params: ServiceProviderListParams,
) -> Result<Vec<ServiceProviderData>, Error> {
    let con: &PgConnection = &*resource.db_pool.get()?;
    let show_all = params.show_all;
    let filter_teams = params.filter_teams;
    let filter_service_variants = params.filter_service_variants;

    let mut query = service_providers::table
        .inner_join(teams::table)
        .inner_join(service_variants::table.inner_join(services::table))
        .order_by((
            service_variants::service_id,
            service_providers::team_id,
            service_providers::published_time.desc(),
        ))
        .distinct_on((service_variants::service_id, service_providers::team_id))
        .into_boxed();

    if !filter_teams.is_empty() {
        query = query.filter(teams::name.eq_any(filter_teams));
    }

    if !filter_service_variants.is_empty() {
        query = query.filter(service_variants::name.eq_any(filter_service_variants));
    }

    if !show_all {
        query = query.filter(services::enabled.eq(true));
    }

    let result = query
        .select((
            teams::name,
            service_variants::name,
            service_providers::connection_string,
            service_providers::published_time,
        ))
        .load(con)?
        .into_iter()
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
    resource: &DaemonResource,
    params: ServiceProviderUpdateParams,
) -> Result<(), Error> {
    let con: &PgConnection = &*resource.db_pool.get()?;
    let team_name = params.team_name;
    let service_variant_name = params.service_variant_name;
    let connection_string = params.connection_string;

    con.transaction::<(), Error, _>(|| {
        let service_variant = service_variants::table
            .filter(service_variants::name.eq(service_variant_name))
            .first::<ServiceVariant>(con)?;

        let team = teams::table
            .filter(teams::name.eq(team_name))
            .first::<Team>(con)?;

        let result = service_providers::table
            .filter(service_providers::team_id.eq(team.id))
            .inner_join(service_variants::table)
            .filter(service_variants::service_id.eq(service_variant.service_id))
            .order_by(service_providers::published_time.desc())
            .first::<(ServiceProvider, ServiceVariant)>(con);

        if let Ok((record, _)) = result {
            let target = service_providers::table.find(record.id);

            if (record.team_id, record.service_variant_id) == (team.id, service_variant.id) {
                diesel::update(target)
                    .set(ServiceProviderChangeset {
                        connection_string: Some(connection_string),
                    })
                    .execute(con)?;
                return Ok(());
            }
        }

        diesel::insert_into(service_providers::table)
            .values(ServiceProviderInsertable {
                team_id: team.id,
                service_variant_id: service_variant.id,
                connection_string,
            })
            .execute(con)?;

        Ok(())
    })
}
