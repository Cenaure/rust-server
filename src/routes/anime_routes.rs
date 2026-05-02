use crate::routes::middlewares::auth_middleware::auth_middleware;
use crate::routes::middlewares::permissions_middleware::require_permissions;
use crate::{handlers, permission};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};
use actix_web::web;

permission!(require_anime_create, "anime_create");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/anime")
            .service(web::resource("/")
                .route(
                    web::get()
                        .to(handlers::anime_handler::get_anime_by_query),
                )
                .route(
                    web::post().wrap(from_fn(auth_middleware))
                        .to(handlers::anime_handler::create_anime)
                        .wrap(from_fn(require_anime_create))
                )
        ).service(
            web::resource("/list").route(
                web::get()
                    .to(handlers::anime_handler::get_anime_list),
            )
        ).service(
            web::resource("/ids/{ids}").route(
                web::get()
                    .to(handlers::anime_handler::get_anime_by_ids),
            )
        ).service(
            web::resource("/search").route(
                web::get()
                    .to(handlers::anime_handler::search_anime_in_local_db),
            )
        ).service(
            web::resource("/random").route(
                web::get()
                    .to(handlers::anime_handler::get_random),
            )
        ).service(
            web::resource("/top").route(
                web::get()
                    .to(handlers::anime_handler::get_top),
            )
        ).service(
            web::resource("/{id}")
                .route(
                    web::get()
                        .to(handlers::anime_handler::get_by_id),
                )
                .route(
                    web::put().wrap(from_fn(auth_middleware))
                        .to(handlers::anime_handler::update_anime)
                        .wrap(from_fn(require_anime_create)),
                )
        )
    );
}