#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tarpc;

extern crate chrono;
extern crate uuid;

use self::model::exploit::{
    ExploitData, ExploitEditParams, ExploitListParams, ExploitRunParams, ExploitStatusParams,
    ExploitTaskData,
};
use self::model::service::provider::{
    ServiceProviderData, ServiceProviderListParams, ServiceProviderUpdateParams,
};
use self::model::service::variant::{
    ServiceVariantAttachmentData, ServiceVariantData, ServiceVariantDownloadParams,
    ServiceVariantEditParams, ServiceVariantListParams,
};
use self::model::service::{ServiceData, ServiceEditParams};
use self::model::team::{TeamData, TeamEditParams};
use uuid::Uuid;

pub mod error;
pub mod model;

service! {
    rpc edit_team(cli_token: Uuid, params: TeamEditParams) -> Result<(), ()>;
    rpc list_team(cli_token: Uuid) -> Result<Vec<TeamData>, ()>;

    rpc edit_service(cli_token: Uuid, params: ServiceEditParams) -> Result<(), ()>;
    rpc list_service(cli_token: Uuid) -> Result<Vec<ServiceData>, ()>;

    rpc download_service_variant(cli_token: Uuid, params: ServiceVariantDownloadParams) -> Result<ServiceVariantAttachmentData, ()>;
    rpc edit_service_variant(cli_token: Uuid, params: ServiceVariantEditParams) -> Result<(), ()>;
    rpc list_service_variant(cli_token: Uuid, params: ServiceVariantListParams) -> Result<Vec<ServiceVariantData>, ()>;

    rpc list_service_provider(cli_token: Uuid, params: ServiceProviderListParams) -> Result<Vec<ServiceProviderData>, ()>;
    rpc update_service_provider(cli_token: Uuid, params: ServiceProviderUpdateParams) -> Result<(), ()>;

    rpc edit_exploit(cli_token: Uuid, params: ExploitEditParams) -> Result<(), ()>;
    rpc list_exploit(cli_token: Uuid, params: ExploitListParams) -> Result<Vec<ExploitData>, ()>;
    rpc run_exploit(cli_token: Uuid, params: ExploitRunParams) -> Result<Option<ExploitTaskData>, ()>;
    rpc stat_exploit(cli_token: Uuid, params: ExploitStatusParams) -> Result<ExploitTaskData, ()>;
}
