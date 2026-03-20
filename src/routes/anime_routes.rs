use crate::handlers;
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/anime").service(
            handlers::anime_handler::get_top
        ).service(
            handlers::anime_handler::get_random
        )
    );
}