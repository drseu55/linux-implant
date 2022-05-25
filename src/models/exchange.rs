use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ExchangeResponse {
    pub public_key: String,
    pub implant_id: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangeRequest {
    pub public_key: String,
}

impl ExchangeRequest {
    pub fn new(public_key: String) -> Self {
        ExchangeRequest { public_key }
    }
}
