#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tarpc;

extern crate chrono;

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
use error::Error;

pub mod error;
pub mod model;

service! {
    rpc edit_team(cli_token: String, params: TeamEditParams) -> () | Error;
    rpc list_team(cli_token: String) -> Vec<TeamData> | Error;

    rpc edit_service(cli_token: String, params: ServiceEditParams) -> () | Error;
    rpc list_service(cli_token: String) -> Vec<ServiceData> | Error;

    rpc download_service_variant(cli_token: String, params: ServiceVariantDownloadParams) -> ServiceVariantAttachmentData | Error;
    rpc edit_service_variant(cli_token: String, params: ServiceVariantEditParams) -> () | Error;
    rpc list_service_variant(cli_token: String, params: ServiceVariantListParams) -> Vec<ServiceVariantData> | Error;

    rpc list_service_provider(cli_token: String, params: ServiceProviderListParams) -> Vec<ServiceProviderData> | Error;
    rpc update_service_provider(cli_token: String, params: ServiceProviderUpdateParams) -> () | Error;

    rpc edit_exploit(cli_token: String, params: ExploitEditParams) -> () | Error;
    rpc list_exploit(cli_token: String, params: ExploitListParams) -> Vec<ExploitData> | Error;
    rpc run_exploit(cli_token: String, params: ExploitRunParams) -> Option<ExploitTaskData> | Error;
    rpc stat_exploit(cli_token: String, params: ExploitStatusParams) -> ExploitTaskData | Error;
}
