use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures_util::TryStreamExt as _;

use crate::utils::{encryption::decrypt_message, steganography::extract_message};

pub async fn decode_message(mut payload: Multipart) -> Result<HttpResponse, Error> {

    // TODO: Implement the decoding logic

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Decoded message!!"))
}
