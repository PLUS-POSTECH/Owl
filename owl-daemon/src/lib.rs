#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate owl_exploit;
extern crate owl_rpc;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;
extern crate tokio;

use self::db::models::*;
use self::db::schema::*;
use self::db::DbPool;
use self::error::Error as DaemonError;
use self::handler::service::provider::{fetch_service_providers, update_service_providers};
use diesel::prelude::*;
use diesel::PgConnection;
use owl_rpc::model::exploit::{
    ExploitData, ExploitEditParams, ExploitListParams, ExploitRunParams, ExploitStatusParams,
    ExploitTaskData,
};
use owl_rpc::model::service::provider::{
    ServiceProviderData, ServiceProviderListParams, ServiceProviderUpdateParams,
};
use owl_rpc::model::service::variant::{
    ServiceVariantAttachmentData, ServiceVariantData, ServiceVariantDownloadParams,
    ServiceVariantEditParams, ServiceVariantListParams,
};
use owl_rpc::model::service::{ServiceData, ServiceEditParams, ServiceListParams};
use owl_rpc::model::team::{TeamData, TeamEditParams};
use owl_rpc::FutureService;
use tarpc::util::Message;
use tokio::runtime::TaskExecutor;

pub mod db;
pub mod error;
pub mod handler;

#[derive(Clone)]
pub struct OwlDaemon {
    db_pool: DbPool,
    task_executor: TaskExecutor,
}

impl OwlDaemon {
    pub fn new(db_pool: DbPool, task_executor: TaskExecutor) -> OwlDaemon {
        OwlDaemon {
            db_pool,
            task_executor,
        }
    }
}

fn run_handler<F, R>(f: F, db_pool: DbPool) -> Result<R, Message>
where
    F: FnOnce(DbPool) -> Result<R, DaemonError>,
{
    match f(db_pool) {
        Ok(result) => Ok(result),
        Err(err) => Err(Message(err.to_string())),
    }
}

fn run_handler_with_param<F, P, R>(f: F, db_pool: DbPool, params: P) -> Result<R, Message>
where
    F: FnOnce(DbPool, P) -> Result<R, DaemonError>,
{
    match f(db_pool, params) {
        Ok(result) => Ok(result),
        Err(err) => Err(Message(err.to_string())),
    }
}

impl FutureService for OwlDaemon {
    // Team
    type EditTeamFut = Result<(), Message>;
    fn edit_team(&self, cli_token: String, params: TeamEditParams) -> Self::EditTeamFut {
        run_handler_with_param(handler::team::edit_team, self.db_pool.clone(), params)
    }

    type ListTeamFut = Result<Vec<TeamData>, Message>;
    fn list_team(&self, cli_token: String) -> Self::ListTeamFut {
        run_handler(handler::team::list_team, self.db_pool.clone())
    }

    // Service
    type EditServiceFut = Result<(), Message>;
    fn edit_service(&self, cli_token: String, params: ServiceEditParams) -> Self::EditServiceFut {
        run_handler_with_param(handler::service::edit_service, self.db_pool.clone(), params)
    }

    type ListServiceFut = Result<Vec<ServiceData>, Message>;
    fn list_service(&self, cli_token: String, params: ServiceListParams) -> Self::ListServiceFut {
        run_handler_with_param(handler::service::list_service, self.db_pool.clone(), params)
    }

    // Service Variant
    type DownloadServiceVariantFut = Result<ServiceVariantAttachmentData, Message>;
    fn download_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantDownloadParams,
    ) -> Self::DownloadServiceVariantFut {
        Err(Message("Not Implemented".to_string()))
    }

    type EditServiceVariantFut = Result<(), Message>;
    fn edit_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantEditParams,
    ) -> Self::EditServiceVariantFut {
        Err(Message("Not Implemented".to_string()))
    }

    type ListServiceVariantFut = Result<Vec<ServiceVariantData>, Message>;
    fn list_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantListParams,
    ) -> Self::ListServiceVariantFut {
        Err(Message("Not Implemented".to_string()))
    }

    // Service Provider
    type ListServiceProviderFut = Result<Vec<ServiceProviderData>, Message>;
    fn list_service_provider(
        &self,
        cli_token: String,
        params: ServiceProviderListParams,
    ) -> Self::ListServiceProviderFut {
        let connection_result = self.db_pool.get();
        if connection_result.is_err() {
            return Err(Message("".to_string()));
        }

        let connection = connection_result.unwrap();
        let result = fetch_service_providers(&*connection, params);

        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(Message("".to_string())),
        }
    }

    type UpdateServiceProviderFut = Result<(), Message>;
    fn update_service_provider(
        &self,
        cli_token: String,
        params: ServiceProviderUpdateParams,
    ) -> Self::UpdateServiceProviderFut {
        let connection_result = self.db_pool.get();
        if connection_result.is_err() {
            return Err(Message("".to_string()));
        }

        let connection = connection_result.unwrap();
        let result = update_service_providers(&*connection, params);

        match result {
            Ok(data) => Ok(data),
            Err(_err) => Err(Message("".to_string())),
        }
    }

    type EditExploitFut = Result<(), Message>;
    fn edit_exploit(&self, cli_token: String, params: ExploitEditParams) -> Self::EditExploitFut {
        Err(Message("Not Implemented".to_string()))
    }

    type ListExploitFut = Result<Vec<ExploitData>, Message>;
    fn list_exploit(&self, cli_token: String, params: ExploitListParams) -> Self::ListExploitFut {
        Err(Message("Not Implemented".to_string()))
    }

    type RunExploitFut = Result<Option<ExploitTaskData>, Message>;
    fn run_exploit(&self, cli_token: String, params: ExploitRunParams) -> Self::RunExploitFut {
        Err(Message("Not Implemented".to_string()))
    }

    type StatExploitFut = Result<ExploitTaskData, Message>;
    fn stat_exploit(&self, cli_token: String, params: ExploitStatusParams) -> Self::StatExploitFut {
        Err(Message("Not Implemented".to_string()))
    }
}

fn test_db(db_pool: DbPool) -> Result<(), DaemonError> {
    info!("Testing DB...");
    let con: &PgConnection = &*db_pool.get()?;

    info!("Insert Test data");
    let insert = diesel::insert_into(teams::table)
        .values((teams::name.eq("PLUS"), teams::description.eq("Best Team")))
        .execute(con)?;
    println!("INSERT: {}", insert);

    info!("Fetch Test data");
    let fetch = teams::table.load::<Team>(con)?;
    for team in fetch {
        println!("FETCH: {} - {}", team.name, team.description);
    }

    info!("Delete Test data");
    let delete = diesel::delete(teams::table)
        .filter(teams::name.eq("PLUS"))
        .execute(con)?;
    println!("DELETE: {}", delete);

    Ok(())
}
