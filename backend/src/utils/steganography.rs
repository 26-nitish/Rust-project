use std::io::Cursor;

use actix_web::error::ErrorInternalServerError;

pub fn embed_message(image_data: &[u8], message: &str) -> Result<Vec<u8>, actix_web::Error> {
    let img = image::load_from_memory(image_data)
        .map_err(|_| ErrorInternalServerError("Failed to load image"))?;

    let mut img_buffer = img.to_rgb8();
    let (width, height) = img_buffer.dimensions();
    let pixels = img_buffer.pixels_mut();

    let message_bytes = message.as_bytes();
    let message_length = message_bytes.len();

    if message_length * 8 > (width * height) as usize {
        return Err(ErrorInternalServerError(
            "Message is too long to encode in this image.",
        ));
    }

    let mut bit_iter = message_bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |bit| (byte >> bit) & 1));

    for pixel in pixels {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        let mut new_b = b & 0xFE; // Make LSB zero

        if let Some(bit) = bit_iter.next() {
            new_b |= bit;
        }

        *pixel = image::Rgb([r, g, new_b]);
    }

    let mut output = Cursor::new(Vec::new());
    img_buffer
        .write_to(&mut output, image::ImageOutputFormat::Png)
        .map_err(|_| ErrorInternalServerError("Failed to write image"))?;

    Ok(output.into_inner())
}

pub fn extract_message(image_data: &[u8]) -> Result<String, actix_web::Error> {
    let img = image::load_from_memory(image_data)
        .map_err(|_| ErrorInternalServerError("Failed to load image"))?;

    let img_buffer = img.to_rgb8();
    let pixels = img_buffer.pixels();

    let mut bits = Vec::new();

    for pixel in pixels {
        let b = pixel[2];
        bits.push(b & 1);
    }

    let bytes = bits
        .chunks(8)
        .map(|chunk| {
            chunk.iter().enumerate().fold(0u8, |acc, (i, &bit)| {
                acc | (bit << (7 - i))
            })
        })
        .take_while(|&byte| byte != 0)
        .collect::<Vec<u8>>();

    let message = String::from_utf8(bytes)
        .map_err(|_| ErrorInternalServerError("Failed to decode message"))?;

    Ok(message)
}
