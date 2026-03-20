use crate::errors::ApiError;
use crate::jikan_integration::endpoints::random::get_random_anime;
use crate::jikan_integration::endpoints::top::get_top_anime;
use crate::models::TopAnimeParams;
use crate::utils::app_config::AppConfig;
use actix_web::{get, web, HttpResponse};

pub const ANIME_COLL_NAME: &str = "anime";

#[get("/top")]
pub async fn get_top(
    config: web::Data<AppConfig>,
    params: web::Query<TopAnimeParams>,
) -> Result<HttpResponse, ApiError> {
    let params = params.into_inner();

    let result = get_top_anime(
        config,
        params.r#type,
        params.filter,
        params.rating,
        params.sfw,
        params.page,
        params.limit,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/random")]
pub async fn get_random(
    config: web::Data<AppConfig>,
) -> Result<HttpResponse, ApiError> {
    let result = get_random_anime(
        config,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}