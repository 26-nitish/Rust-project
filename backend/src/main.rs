use actix_web::{middleware, App, HttpServer};
use actix_web::web;
use actix_cors::Cors;
use actix_web::http;

mod handlers {
    pub mod encode;
    pub mod decode;
}
mod utils {
    pub mod encryption;
    pub mod steganography;
}

use crate::handlers::{decode::decode_message, encode::encode_message};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/encode")
            .route(web::post().to(encode_message))
    )
    .service(
        web::resource("/decode")
            .route(web::post().to(decode_message))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Starting server at http://localhost:3333");

    HttpServer::new(|| {
        App::new()
            //.wrap(
              /*  Cors::default()
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .allow_any_header()
                    .max_age(3600)
            )*/
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )            
            .wrap(middleware::Logger::default())
            .configure(init_routes)
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
