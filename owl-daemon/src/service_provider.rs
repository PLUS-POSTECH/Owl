use chrono::{DateTime, Utc};
use db::schema::*;
use diesel::prelude::*;
use diesel::PgConnection;
use error::Error;
use owl_rpc::model::service::provider::{
    ServiceProviderData, ServiceProviderListParams, ServiceProviderUpdateParams,
};

pub fn fetch_service_providers(
    conn: &PgConnection,
    params: ServiceProviderListParams,
) -> Result<Vec<ServiceProviderData>, Error> {
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
            .load(&*conn)
    } else {
        query
            .select((
                teams::name,
                service_variants::name,
                service_providers::connection_string,
                service_providers::published_time,
            ))
            .load(&*conn)
    }?.into_iter()
        .map(
            |x: (String, String, String, DateTime<Utc>)| ServiceProviderData {
                provider_name: x.0,
                service_variant_name: x.1,
                connection_string: x.2,
                published_time: x.3,
            },
        )
        .collect();
    Ok(result)
}
