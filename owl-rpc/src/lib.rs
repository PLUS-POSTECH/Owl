#![feature(plugin, use_extern_macros, proc_macro_path_invoc)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;

service! {
    rpc hello(name: String) -> String;
}
