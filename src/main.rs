use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use log::info;
use mongodb::Client;

mod models;
mod errors;
mod routes;
mod handlers;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let client = Client::with_uri_str(database_url).await.expect("Failed connecting to database");

    let port = 8080;

    info!("Server starting on port {port}");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(client.clone()))
            .service(
                web::scope("/api")
                    .configure(routes::users_routes::config)
                    .configure(routes::auth_routes::config)
                )
            })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}