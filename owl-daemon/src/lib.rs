#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

extern crate chrono;
extern crate digest;
extern crate dotenv;
extern crate futures;
extern crate owl_exploit;
extern crate owl_rpc;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate sha3;
extern crate tarpc;
extern crate tokio;

use self::db::DbPool;
use self::error::Error as DaemonError;
use owl_rpc::model::exploit::*;
use owl_rpc::model::service::provider::*;
use owl_rpc::model::service::variant::*;
use owl_rpc::model::service::*;
use owl_rpc::model::team::*;
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
        run_handler_with_param(
            handler::service::variant::download_service_variant,
            self.db_pool.clone(),
            params,
        )
    }

    type EditServiceVariantFut = Result<(), Message>;
    fn edit_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantEditParams,
    ) -> Self::EditServiceVariantFut {
        run_handler_with_param(
            handler::service::variant::edit_service_variant,
            self.db_pool.clone(),
            params,
        )
    }

    type ListServiceVariantFut = Result<Vec<ServiceVariantData>, Message>;
    fn list_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantListParams,
    ) -> Self::ListServiceVariantFut {
        run_handler_with_param(
            handler::service::variant::list_service_variant,
            self.db_pool.clone(),
            params,
        )
    }

    // Service Provider
    type ListServiceProviderFut = Result<Vec<ServiceProviderData>, Message>;
    fn list_service_provider(
        &self,
        cli_token: String,
        params: ServiceProviderListParams,
    ) -> Self::ListServiceProviderFut {
        run_handler_with_param(
            handler::service::provider::list_service_provider,
            self.db_pool.clone(),
            params,
        )
    }

    type UpdateServiceProviderFut = Result<(), Message>;
    fn update_service_provider(
        &self,
        cli_token: String,
        params: ServiceProviderUpdateParams,
    ) -> Self::UpdateServiceProviderFut {
        run_handler_with_param(
            handler::service::provider::update_service_provider,
            self.db_pool.clone(),
            params,
        )
    }

    type DownloadExploitFut = Result<ExploitAttachmentData, Message>;
    fn download_exploit(
        &self,
        cli_token: String,
        params: ExploitDownloadParams,
    ) -> Self::DownloadExploitFut {
        run_handler_with_param(
            handler::exploit::download_exploit,
            self.db_pool.clone(),
            params,
        )
    }

    type EditExploitFut = Result<(), Message>;
    fn edit_exploit(&self, cli_token: String, params: ExploitEditParams) -> Self::EditExploitFut {
        run_handler_with_param(handler::exploit::edit_exploit, self.db_pool.clone(), params)
    }

    type ListExploitFut = Result<Vec<ExploitData>, Message>;
    fn list_exploit(&self, cli_token: String, params: ExploitListParams) -> Self::ListExploitFut {
        run_handler_with_param(handler::exploit::list_exploit, self.db_pool.clone(), params)
    }

    type RunExploitFut = Result<Option<Vec<ExploitTaskData>>, Message>;
    fn run_exploit(&self, cli_token: String, params: ExploitRunParams) -> Self::RunExploitFut {
        Err(Message("Not Implemented".to_string()))
    }

    type RunAllExploitFut = Result<(), Message>;
    fn run_all_exploit(&self, cli_token: String) -> Self::RunAllExploitFut {
        Err(Message("Not Implemented".to_string()))
    }

    type StatusExploitFut = Result<Vec<ExploitTaskData>, Message>;
    fn status_exploit(
        &self,
        cli_token: String,
        params: ExploitStatusParams,
    ) -> Self::StatusExploitFut {
        Err(Message("Not Implemented".to_string()))
    }
}
