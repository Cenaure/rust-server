use actix_web::web;
use crate::{handlers, permission};
use crate::routes::middlewares::permissions_middleware::require_permissions;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};

permission!(require_anime_create, "anime_create");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/producers")
            .service(web::resource("/").route(
                web::get()
                    .to(handlers::anime_producers_handler::get_anime_list),
            ))
            .service(web::resource("/{mal_id}").route(
                web::get()
                    .to(handlers::anime_producers_handler::get_producer_by_mal_id),
            ).route(
                web::patch()
                    .to(handlers::anime_producers_handler::update_producer)))
                .wrap(from_fn(require_anime_create))
    );
}