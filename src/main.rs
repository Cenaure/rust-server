use crate::utils::app_config::AppConfig;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use log::info;
use mongodb::Client;
use utoipa_swagger_ui::SwaggerUi;

use utoipa::OpenApi;

mod errors;
mod handlers;
mod models;
mod routes;
mod utils;
pub mod jikan_integration;
mod openapi;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    dotenvy::dotenv().ok();

    // Extracting config from .env file
    let config = AppConfig {
        jwt_secret: std::env::var("ACCESS_TOKEN_SECRET")
            .expect("ACCESS_TOKEN_SECRET must be set").into_bytes(),
        database_url: std::env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file"),
        jikan_api_url: std::env::var("JIKAN_API_URL")
            .expect("Jikan Api Url is not set in .env file"),
        // Client for fetching data from jikan
        http_client: reqwest::Client::new(),
    };

    let client = Client::with_uri_str(config.database_url.clone()).await.expect("Failed connecting to database");

    let port = 8080;

    info!("Server starting on port {port}");

    HttpServer::new(move || {
        let logger = Logger::default();

        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH", "OPTIONS"])
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
            )
            .service(
                web::scope("/api")
                    .wrap(cors)
                    .configure(routes::users_routes::config)
                    .configure(routes::groups_routes::config)
                    .configure(routes::auth_routes::config)
                    .configure(routes::anime_routes::config))
            .wrap(logger)
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
