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

pub mod error;
pub mod model;

service! {
    rpc edit_team(cli_token: String, params: TeamEditParams) -> Result<(), ()>;
    rpc list_team(cli_token: String) -> Result<Vec<TeamData>, ()>;

    rpc edit_service(cli_token: String, params: ServiceEditParams) -> Result<(), ()>;
    rpc list_service(cli_token: String) -> Result<Vec<ServiceData>, ()>;

    rpc download_service_variant(cli_token: String, params: ServiceVariantDownloadParams) -> Result<ServiceVariantAttachmentData, ()>;
    rpc edit_service_variant(cli_token: String, params: ServiceVariantEditParams) -> Result<(), ()>;
    rpc list_service_variant(cli_token: String, params: ServiceVariantListParams) -> Result<Vec<ServiceVariantData>, ()>;

    rpc list_service_provider(cli_token: String, params: ServiceProviderListParams) -> Result<Vec<ServiceProviderData>, ()>;
    rpc update_service_provider(cli_token: String, params: ServiceProviderUpdateParams) -> Result<(), ()>;

    rpc edit_exploit(cli_token: String, params: ExploitEditParams) -> Result<(), ()>;
    rpc list_exploit(cli_token: String, params: ExploitListParams) -> Result<Vec<ExploitData>, ()>;
    rpc run_exploit(cli_token: String, params: ExploitRunParams) -> Result<Option<ExploitTaskData>, ()>;
    rpc stat_exploit(cli_token: String, params: ExploitStatusParams) -> Result<ExploitTaskData, ()>;
}
