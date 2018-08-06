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
    ExploitEditParams, ExploitListItem, ExploitListParams, ExploitRunParams, ExploitStatusItem,
    ExploitStatusParams,
};
use self::model::service::provider::{
    ServiceProviderListItem, ServiceProviderListParams, ServiceProviderUpdateParams,
};
use self::model::service::variant::{
    ServiceVariantData, ServiceVariantEditParams, ServiceVariantFetchParams,
    ServiceVariantListItem, ServiceVariantListParams,
};
use self::model::service::{ServiceEditParams, ServiceListItem};
use self::model::team::{TeamEditParams, TeamListItem};
use uuid::Uuid;

pub mod error;
pub mod model;

service! {
    rpc edit_team(cli_token: Uuid, params: TeamEditParams) -> Result<(), ()>;
    rpc list_team(cli_token: Uuid) -> Result<Vec<TeamListItem>, ()>;

    rpc edit_service(cli_token: Uuid, params: ServiceEditParams) -> Result<(), ()>;
    rpc list_service(cli_token: Uuid) -> Result<Vec<ServiceListItem>, ()>;

    rpc edit_service_variant(cli_token: Uuid, params: ServiceVariantEditParams) -> Result<(), ()>;
    rpc fetch_service_variant(cli_token: Uuid, params: ServiceVariantFetchParams) -> Result<ServiceVariantData, ()>;
    rpc list_service_variant(cli_token: Uuid, params: ServiceVariantListParams) -> Result<Vec<ServiceVariantListItem>, ()>;

    rpc list_service_provider(cli_token: Uuid, params: ServiceProviderListParams) -> Result<Vec<ServiceProviderListItem>, ()>;
    rpc update_service_provider(cli_token: Uuid, params: ServiceProviderUpdateParams) -> Result<(), ()>;

    rpc edit_exploit(cli_token: Uuid, params: ExploitEditParams) -> Result<(), ()>;
    rpc list_exploit(cli_token: Uuid, params: ExploitListParams) -> Result<Vec<ExploitListItem>, ()>;
    rpc run_exploit(cli_token: Uuid, params: ExploitRunParams) -> Result<Option<ExploitStatusItem>, ()>;
    rpc stat_exploit(cli_token: Uuid, params: ExploitStatusParams) -> Result<ExploitStatusItem, ()>;
}
