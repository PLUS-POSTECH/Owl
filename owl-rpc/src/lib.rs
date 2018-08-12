#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tarpc;

extern crate chrono;

use self::model::exploit::*;
use self::model::service::provider::*;
use self::model::service::variant::*;
use self::model::service::*;
use self::model::team::*;
use tarpc::util::Message;

pub mod model;

service! {
    rpc edit_team(cli_token: String, params: TeamEditParams) -> () | Message;
    rpc list_team(cli_token: String) -> Vec<TeamData> | Message;

    rpc edit_service(cli_token: String, params: ServiceEditParams) -> () | Message;
    rpc list_service(cli_token: String, params: ServiceListParams) -> Vec<ServiceData> | Message;

    rpc download_service_variant(cli_token: String, params: ServiceVariantDownloadParams) -> ServiceVariantAttachmentData | Message;
    rpc add_service_variant(cli_token: String, params: ServiceVariantAddParams) -> String | Message;
    rpc edit_service_variant(cli_token: String, params: ServiceVariantEditParams) -> () | Message;
    rpc list_service_variant(cli_token: String, params: ServiceVariantListParams) -> Vec<ServiceVariantData> | Message;

    rpc list_service_provider(cli_token: String, params: ServiceProviderListParams) -> Vec<ServiceProviderData> | Message;
    rpc update_service_provider(cli_token: String, params: ServiceProviderUpdateParams) -> () | Message;

    rpc download_exploit(cli_token: String, params: ExploitDownloadParams) -> ExploitAttachmentData | Message;
    rpc add_exploit(cli_token: String, params: ExploitAddParams) -> String | Message;
    rpc edit_exploit(cli_token: String, params: ExploitEditParams) -> () | Message;
    rpc failure_exploit(cli_token: String, params: ExploitFailureParams) -> () | Message;
    rpc list_exploit(cli_token: String, params: ExploitListParams) -> Vec<ExploitData> | Message;
    rpc run_exploit(cli_token: String, params: ExploitRunParams) -> Option<Vec<ExploitTaskData>> | Message;
    rpc run_all_exploit(cli_token: String) -> () | Message;
    rpc status_exploit(cli_token: String, params: ExploitStatusParams) -> Vec<ExploitTaskData> | Message;
}
