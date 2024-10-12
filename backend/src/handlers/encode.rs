use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures_util::TryStreamExt as _;

use crate::utils::{encryption::encrypt_message, steganography::embed_message};

pub async fn encode_message(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut image_data = None;
    let mut secret_message = None;
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
            Some("message") => {
                let mut msg = Vec::new();
                while let Some(chunk) = field.try_next().await? {
                    msg.extend_from_slice(&chunk);
                }
                secret_message = Some(String::from_utf8(msg).unwrap());
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

    let mut message = secret_message.ok_or_else(|| {
        actix_web::error::ErrorBadRequest("Secret message is required.")
    })?;

    if let Some(pwd) = password {
        message = encrypt_message(&message, &pwd);
    }

    let stego_image = embed_message(&image_data, &message)?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(stego_image))
}
