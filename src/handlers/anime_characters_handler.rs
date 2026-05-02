use actix_web::{web, HttpResponse};
use tracing::info;
use crate::errors::ApiError;
use crate::jikan_integration::common::structs::character::AnimeCharactersResponse;
use crate::services::anime_service;
use crate::utils::app_config::AppConfig;
#[utoipa::path(
    get,
    path = "/api/characters/{id}",
    tag = "Characters",
    responses(
        (status = 200, description = "Characters by anime id", body = AnimeCharactersResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error")
    )
)]
pub async fn get_characters(
    config: web::Data<AppConfig>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::get_characters_service(config.clone(), path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}