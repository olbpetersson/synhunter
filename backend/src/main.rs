#[macro_use]
extern crate error_chain;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate jsonrpc_core;
extern crate jsonrpc_pubsub;
#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_ws_server;
extern crate uuid;
extern crate rand;

mod structs;
mod server;
mod wordlist;


fn main() {
    let server = server::create_server();
    server.wait().unwrap();
}

