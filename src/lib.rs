#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

#[cfg(feature = "cli")]
#[macro_use]
extern crate prettytable;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub mod clarity;
pub mod repl;
pub mod contracts;

struct GlobalContext {
    session: Option<Session>,
}

static mut WASM_GLOBAL_CONTEXT: GlobalContext = GlobalContext { session: None };

#[cfg(feature = "cli")]
pub mod frontend;

#[cfg(feature = "cli")]
pub use frontend::Terminal;

use repl::{Session, SessionSettings};
use repl::settings::InitialLink;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub async fn init_session(fetch_contract: String) -> String {

    let (session, output) = unsafe { match WASM_GLOBAL_CONTEXT.session.take() {
        Some(session) => (session, "".to_string()),
        None => {
            let initial_links = if fetch_contract == "null" {
                vec![]
            } else {
                vec![InitialLink {
                    contract_id: fetch_contract.to_string(),
                    stacks_node_addr: None,
                    cache: None,
                }]
            };
            let mut settings = SessionSettings::default();
            settings.include_boot_contracts = vec!["costs".into()];
            settings.initial_links = initial_links;
            let mut session = Session::new(settings);
            let output = session.start_wasm().await;
            (session, output)
        }
    }};

    unsafe {
        WASM_GLOBAL_CONTEXT.session = Some(session);
    }
    output
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn handle_command(command: &str) -> String {

    let mut session = unsafe { match WASM_GLOBAL_CONTEXT.session.take() {
        Some(session) => session,
        None => {
            return "Error: session lost".to_string()
        }
    }};

    let output_lines = session.handle_command(command);

    unsafe {
        WASM_GLOBAL_CONTEXT.session = Some(session);
    }

    output_lines.join("\n").to_string()
}
