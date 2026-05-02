use actix_web::web;
use crate::{handlers};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/characters")
            .service(web::resource("/").route(
                web::get()
            ))
            .service(
                web::resource("/{id}").route(
                    web::get()
                        .to(handlers::anime_characters_handler::get_characters),
                )
            )
    );
}