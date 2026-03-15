use crate::handlers;
use crate::routes::middlewares::auth_middleware::auth_middleware;
use actix_web::middleware::from_fn;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/groups")
        .wrap(from_fn(auth_middleware))
        .service(handlers::groups_handler::list_groups)
    );
}