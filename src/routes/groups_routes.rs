use crate::routes::middlewares::auth_middleware::auth_middleware;
use crate::routes::middlewares::permissions_middleware::require_permissions;
use crate::{handlers, permission};
use actix_web::body::MessageBody;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::from_fn;
use actix_web::middleware::Next;
use actix_web::web;

permission!(require_list_users, "list_users");
permission!(require_create_groups, "create_groups");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/groups")
        .wrap(from_fn(auth_middleware))
        .service(
            web::resource("/")
                .route(
                    web::get()
                        .to(handlers::groups_handler::list_groups)
                        .wrap(from_fn(require_list_users))
                )
                .route(
                    web::post()
                        .to(handlers::groups_handler::add_group)
                        .wrap(from_fn(require_create_groups))
                )

        )
        .service(
            web::resource("/{id}")
                .route(
                    web::delete()
                        .to(handlers::groups_handler::delete_group)
                        .wrap(from_fn(require_create_groups))
                )
                .route(
                    web::patch()
                        .to(handlers::groups_handler::patch_group)
                        .wrap(from_fn(require_create_groups))
                )
                .route(
                    web::get()
                        .to(handlers::groups_handler::get_group)
                        .wrap(from_fn(require_create_groups))
                )
        )
    );
}