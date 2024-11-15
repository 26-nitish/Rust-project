use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures_util::TryStreamExt as _;

use crate::utils::{encryption::decrypt_message, steganography::extract_message};

pub async fn decode_message(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut image_data = None;
    let mut password = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();

        match content_disposition.get_name() {
            Some("image") => {
                let mut data = Vec::new();
                while let Some(chunk) = field.try_next().await? {
                    data.extend_from_slice(&chunk);
                }
                image_data = Some(data);
            }
            Some("password") => {
                let mut pwd = Vec::new();
                while let Some(chunk) = field.try_next().await? {
                    pwd.extend_from_slice(&chunk);
                }
                password = Some(String::from_utf8(pwd).unwrap());
            }
            _ => {}
        }
    }

    let image_data = image_data.ok_or_else(|| {
        actix_web::error::ErrorBadRequest("Image data is required.")
    })?;

    let mut message = extract_message(&image_data)?;

    if let Some(pwd) = password {
        message = decrypt_message(&message, &pwd);
    }

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(message))
}
