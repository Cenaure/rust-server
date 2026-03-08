use crate::handlers;
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth").service(
            handlers::auth_handler::sign_in
        ).service(
            handlers::auth_handler::sign_up
        ).service(
            handlers::auth_handler::logout
        )
    );
}