use base64;
use reqwest;
use x25519_dalek::PublicKey;

use crate::errors::ImplantError;
use crate::models::exchange::{ExchangeRequest, ExchangeResponse};
use crate::{HOST, PORT, PROTOCOL};

const EXCHANGE_ENDPOINT: &str = "/api/exchange";

pub fn exchange_keys(public_key: PublicKey) -> Result<ExchangeResponse, ImplantError> {
    let full_url = format!("{}://{}:{}{}", PROTOCOL, HOST, PORT, EXCHANGE_ENDPOINT);

    let base64_public_key = base64::encode(public_key.as_bytes());

    let exchange_public_key = ExchangeRequest::new(base64_public_key);

    let client = reqwest::blocking::Client::new();

    let response: ExchangeResponse = client
        .post(full_url)
        .json(&exchange_public_key)
        .send()?
        .json()?;

    Ok(response)
}
