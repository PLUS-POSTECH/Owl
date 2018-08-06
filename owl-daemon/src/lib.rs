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
use self::service_provider::fetch_service_providers;
use diesel::prelude::*;
use diesel::PgConnection;
use owl_rpc::error::Error as RpcError;
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
use owl_rpc::model::service::{ServiceData, ServiceEditParams};
use owl_rpc::model::team::{TeamData, TeamEditParams};
use owl_rpc::FutureService;
use tokio::runtime::TaskExecutor;

pub mod db;
pub mod error;
pub mod service_provider;

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

impl FutureService for OwlDaemon {
    type EditTeamFut = Result<(), RpcError>;
    fn edit_team(&self, cli_token: String, params: TeamEditParams) -> Self::EditTeamFut {
        Err(())
    }

    type ListTeamFut = Result<Vec<TeamData>, RpcError>;
    fn list_team(&self, cli_token: String) -> Self::ListTeamFut {
        Err(())
    }

    type EditServiceFut = Result<(), RpcError>;
    fn edit_service(&self, cli_token: String, params: ServiceEditParams) -> Self::EditServiceFut {
        Err(())
    }

    type ListServiceFut = Result<Vec<ServiceData>, RpcError>;
    fn list_service(&self, cli_token: String) -> Self::ListServiceFut {
        Err(())
    }

    type DownloadServiceVariantFut = Result<ServiceVariantAttachmentData, RpcError>;
    fn download_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantDownloadParams,
    ) -> Self::DownloadServiceVariantFut {
        Err(())
    }

    type EditServiceVariantFut = Result<(), RpcError>;
    fn edit_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantEditParams,
    ) -> Self::EditServiceVariantFut {
        Err(())
    }

    type ListServiceVariantFut = Result<Vec<ServiceVariantData>, RpcError>;
    fn list_service_variant(
        &self,
        cli_token: String,
        params: ServiceVariantListParams,
    ) -> Self::ListServiceVariantFut {
        Err(())
    }

    type ListServiceProviderFut = Result<Vec<ServiceProviderData>, RpcError>;
    fn list_service_provider(
        &self,
        cli_token: String,
        params: ServiceProviderListParams,
    ) -> Self::ListServiceProviderFut {
        let connection_result = self.db_pool.get();
        if connection_result.is_err() {
            return Err(())
        }

        let connection = connection_result.unwrap();
        let result = fetch_service_providers(&*connection, params);

        match result {
            Ok(data) => Ok(data),
            Err(_err) => Err(()),
        }
    }

    type UpdateServiceProviderFut = Result<(), RpcError>;
    fn update_service_provider(
        &self,
        cli_token: String,
        params: ServiceProviderUpdateParams,
    ) -> Self::UpdateServiceProviderFut {
        Err(())
    }

    type EditExploitFut = Result<(), RpcError>;
    fn edit_exploit(&self, cli_token: String, params: ExploitEditParams) -> Self::EditExploitFut {
        Err(())
    }

    type ListExploitFut = Result<Vec<ExploitData>, RpcError>;
    fn list_exploit(&self, cli_token: String, params: ExploitListParams) -> Self::ListExploitFut {
        Err(())
    }

    type RunExploitFut = Result<Option<ExploitTaskData>, RpcError>;
    fn run_exploit(&self, cli_token: String, params: ExploitRunParams) -> Self::RunExploitFut {
        Err(())
    }

    type StatExploitFut = Result<ExploitTaskData, RpcError>;
    fn stat_exploit(&self, cli_token: String, params: ExploitStatusParams) -> Self::StatExploitFut {
        Err(())
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
