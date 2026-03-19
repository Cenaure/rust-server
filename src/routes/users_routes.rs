use crate::routes::middlewares::auth_middleware::auth_middleware;
use crate::routes::middlewares::permissions_middleware::require_permissions;
use crate::{handlers, permission};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};
use actix_web::web;

permission!(require_list_users, "list_users");
permission!(require_create_user, "create_user");
permission!(require_delete_user, "delete_user");
permission!(require_patch_user, "patch_user");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(from_fn(auth_middleware))
            .service(
                web::resource("/")
                    .route(
                        web::post()
                            .to(handlers::users_handler::add_user)
                            .wrap(from_fn(require_create_user)),
                    )
                    .route(
                        web::get()
                            .to(handlers::users_handler::list_users)
                            .wrap(from_fn(require_list_users)),
                    )
            )
            .service(
                web::resource("/{id}")
                    .route(
                        web::patch()
                            .to(handlers::users_handler::patch_user)
                            .wrap(from_fn(require_patch_user)),
                    )
                    .route(
                        web::delete()
                            .to(handlers::users_handler::delete_user)
                            .wrap(from_fn(require_delete_user)),
                    )
                    .route(
                        web::get()
                            .to(handlers::users_handler::get_user)
                            .wrap(from_fn(require_list_users)),
                    ),
            ),
    );
}