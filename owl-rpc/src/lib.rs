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
    rpc edit_service_variant(cli_token: String, params: ServiceVariantEditParams) -> () | Message;
    rpc list_service_variant(cli_token: String, params: ServiceVariantListParams) -> Vec<ServiceVariantData> | Message;

    rpc list_service_provider(cli_token: String, params: ServiceProviderListParams) -> Vec<ServiceProviderData> | Message;
    rpc update_service_provider(cli_token: String, params: ServiceProviderUpdateParams) -> () | Message;

    rpc edit_exploit(cli_token: String, params: ExploitEditParams) -> () | Message;
    rpc list_exploit(cli_token: String, params: ExploitListParams) -> Vec<ExploitData> | Message;
    rpc run_exploit(cli_token: String, params: ExploitRunParams) -> Option<ExploitTaskData> | Message;
    rpc stat_exploit(cli_token: String, params: ExploitStatusParams) -> ExploitTaskData | Message;
}
