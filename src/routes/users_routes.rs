use crate::handlers;
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users")
        .service(handlers::users_handler::list_users)
        .service(handlers::users_handler::add_user)
    );
}