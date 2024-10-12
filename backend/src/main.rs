use actix_web::{middleware, App, HttpServer};
use actix_web::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Starting server at http://localhost:3333");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(init_routes)
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}