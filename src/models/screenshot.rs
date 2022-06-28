use serde::Serialize;

#[derive(Serialize)]
pub struct ScreenshotResponse {
    pub bitflipped_bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ScreenshotResponse {
    pub fn new(bitflipped_bytes: Vec<u8>, width: u32, height: u32) -> Self {
        ScreenshotResponse {
            bitflipped_bytes,
            width,
            height,
        }
    }
}
