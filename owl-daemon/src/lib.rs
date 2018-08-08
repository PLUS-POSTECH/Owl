#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate owl_exploit;
extern crate owl_rpc;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tarpc;
extern crate tokio;
extern crate toml;

use self::db::DbPool;
use self::error::Error as DaemonError;
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

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: Server,
}

#[derive(Clone, Deserialize)]
pub struct Server {
    pub connection: String,
    pub db: String,
    pub user_tokens: Vec<String>,
    pub admin_tokens: Vec<String>,
}

#[derive(Clone)]
pub struct DaemonResource {
    pub db_pool: DbPool,
    pub task_executor: TaskExecutor,
    pub config: Config,
}

#[derive(Clone)]
pub struct OwlDaemon {
    pub resource: DaemonResource,
}

impl OwlDaemon {
    pub fn new(db_pool: DbPool, task_executor: TaskExecutor, config: Config) -> OwlDaemon {
        OwlDaemon {
            resource: DaemonResource {
                db_pool,
                task_executor,
                config,
            },
        }
    }
}

fn run_handler<F, R>(f: F, resource: &DaemonResource) -> Result<R, Message>
where
    F: FnOnce(&DaemonResource) -> Result<R, DaemonError>,
{
    match f(resource) {
        Ok(result) => Ok(result),
        Err(err) => Err(Message(err.to_string())),
    }
}

fn run_handler_with_param<F, P, R>(f: F, resource: &DaemonResource, params: P) -> Result<R, Message>
where
    F: FnOnce(&DaemonResource, P) -> Result<R, DaemonError>,
{
    match f(resource, params) {
        Ok(result) => Ok(result),
        Err(err) => Err(Message(err.to_string())),
    }
}

impl FutureService for OwlDaemon {
    // Team
    type EditTeamFut = Result<(), Message>;
    fn edit_team(&self, cli_token: String, params: TeamEditParams) -> Self::EditTeamFut {
        run_handler_with_param(handler::team::edit_team, &self.resource, params)
    }

    type ListTeamFut = Result<Vec<TeamData>, Message>;
    fn list_team(&self, cli_token: String) -> Self::ListTeamFut {
        run_handler(handler::team::list_team, &self.resource)
    }

    // Service
    type EditServiceFut = Result<(), Message>;
    fn edit_service(&self, cli_token: String, params: ServiceEditParams) -> Self::EditServiceFut {
        run_handler_with_param(handler::service::edit_service, &self.resource, params)
    }

    type ListServiceFut = Result<Vec<ServiceData>, Message>;
    fn list_service(&self, cli_token: String, params: ServiceListParams) -> Self::ListServiceFut {
        run_handler_with_param(handler::service::list_service, &self.resource, params)
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
        Err(Message("Not Implemented".to_string()))
    }

    type UpdateServiceProviderFut = Result<(), Message>;
    fn update_service_provider(
        &self,
        cli_token: String,
        params: ServiceProviderUpdateParams,
    ) -> Self::UpdateServiceProviderFut {
        Err(Message("Not Implemented".to_string()))
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
