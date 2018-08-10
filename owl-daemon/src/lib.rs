#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

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
extern crate toml;

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

enum Permission {
    Admin,
    User,
}

fn check_permission(
    permission: Permission,
    cli_token: String,
    resource: &DaemonResource,
) -> Result<(), DaemonError> {
    match permission {
        Permission::Admin => if resource.config.server.admin_tokens.contains(&cli_token)
            || resource.config.server.user_tokens.contains(&cli_token)
        {
            Ok(())
        } else {
            Err(DaemonError::PermissionError)
        },
        Permission::User => if resource.config.server.user_tokens.contains(&cli_token) {
            Ok(())
        } else {
            Err(DaemonError::PermissionError)
        },
    }
}

fn run_handler<F, R>(
    permission: Permission,
    cli_token: String,
    f: F,
    resource: &DaemonResource,
) -> Result<R, Message>
where
    F: FnOnce(&DaemonResource) -> Result<R, DaemonError>,
{
    if let Err(err) = check_permission(permission, cli_token, resource) {
        return Err(Message(err.to_string()));
    };
    match f(resource) {
        Ok(result) => Ok(result),
        Err(err) => Err(Message(err.to_string())),
    }
}

fn run_handler_with_param<F, P, R>(
    permission: Permission,
    cli_token: String,
    f: F,
    resource: &DaemonResource,
    params: P,
) -> Result<R, Message>
where
    F: FnOnce(&DaemonResource, P) -> Result<R, DaemonError>,
{
    if let Err(err) = check_permission(permission, cli_token, resource) {
        return Err(Message(err.to_string()));
    };
    match f(resource, params) {
        Ok(result) => Ok(result),
        Err(err) => Err(Message(err.to_string())),
    }
}

impl FutureService for OwlDaemon {
    // Team
    type EditTeamFut = Result<(), Message>;
    fn edit_team(&self, cli_token: String, params: TeamEditParams) -> Self::EditTeamFut {
        run_handler_with_param(
            Permission::Admin,
            cli_token,
            handler::team::edit_team,
            &self.resource,
            params,
        )
    }

    type ListTeamFut = Result<Vec<TeamData>, Message>;
    fn list_team(&self, cli_token: String) -> Self::ListTeamFut {
        run_handler(
            Permission::User,
            cli_token,
            handler::team::list_team,
            &self.resource,
        )
    }

    // Service
    type EditServiceFut = Result<(), Message>;
    fn edit_service(&self, cli_token: String, params: ServiceEditParams) -> Self::EditServiceFut {
        run_handler_with_param(
            Permission::Admin,
            cli_token,
            handler::service::edit_service,
            &self.resource,
            params,
        )
    }

    type ListServiceFut = Result<Vec<ServiceData>, Message>;
    fn list_service(&self, cli_token: String, params: ServiceListParams) -> Self::ListServiceFut {
        run_handler_with_param(
            Permission::User,
            cli_token,
            handler::service::list_service,
            &self.resource,
            params,
        )
    }

    // Service Variant
    type DownloadServiceVariantFut = Result<ServiceVariantAttachmentData, Message>;
    fn download_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantDownloadParams,
    ) -> Self::DownloadServiceVariantFut {
        run_handler_with_param(
            Permission::User,
            cli_token,
            handler::service::variant::download_service_variant,
            &self.resource,
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
            Permission::Admin,
            cli_token,
            handler::service::variant::edit_service_variant,
            &self.resource,
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
            Permission::User,
            cli_token,
            handler::service::variant::list_service_variant,
            &self.resource,
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
            Permission::User,
            cli_token,
            handler::service::provider::list_service_provider,
            &self.resource,
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
            Permission::Admin,
            cli_token,
            handler::service::provider::update_service_provider,
            &self.resource,
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
            Permission::User,
            cli_token,
            handler::exploit::download_exploit,
            &self.resource,
            params,
        )
    }

    type EditExploitFut = Result<(), Message>;
    fn edit_exploit(&self, cli_token: String, params: ExploitEditParams) -> Self::EditExploitFut {
        run_handler_with_param(
            Permission::User,
            cli_token,
            handler::exploit::edit_exploit,
            &self.resource,
            params,
        )
    }

    type ListExploitFut = Result<Vec<ExploitData>, Message>;
    fn list_exploit(&self, cli_token: String, params: ExploitListParams) -> Self::ListExploitFut {
        run_handler_with_param(
            Permission::User,
            cli_token,
            handler::exploit::list_exploit,
            &self.resource,
            params,
        )
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
        run_handler_with_param(
            Permission::User,
            cli_token,
            handler::exploit::status_exploit,
            &self.resource,
            params,
        )
    }
}
