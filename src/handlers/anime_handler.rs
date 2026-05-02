use crate::errors::ApiError;
use crate::models::{AnimeListParams, AnimeSearchParams, CreateAnimeRequest, TopAnimeParams, UpdateAnimeRequest};
use crate::utils::app_config::AppConfig;
use actix_web::{web, HttpResponse};
use mongodb::Client;
use crate::jikan_integration::common::structs::anime::{AnimeByIdResponse, AnimeSearchResponse};
use crate::jikan_integration::common::structs::top::AnimeTopJikanResponse;
use crate::jikan_integration::common::structs::random::AnimeRandomJikanResponse;
use crate::services::anime_service;

// Endpoints
#[utoipa::path(
    get,
    path = "/api/anime/top",
    tag = "Anime",
    params(TopAnimeParams),
    responses(
        (status = 200, description = "Top anime list",  body = AnimeTopJikanResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_top(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
    params: web::Query<TopAnimeParams>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::get_top_anime_service(config.clone(), client.get_ref(), params.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/random",
    tag = "Anime",
    responses(
        (status = 200, description = "Random anime", body = AnimeRandomJikanResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_random(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::get_random_anime_service(config.clone(), client.get_ref()).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/{id}",
    tag = "Anime",
    responses(
        (status = 200, description = "Anime by id",     body = AnimeByIdResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_by_id(
    client: web::Data<Client>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::get_anime_by_id_service(
        client.get_ref(),
        path.into_inner()
    ).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/ids/{ids}",
    tag = "Anime",
    responses(
        (status = 200, description = "Anime by ids", body = AnimeSearchResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_anime_by_ids(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::get_anime_by_ids_service(client.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "Anime",
    responses(
        (status = 200, description = "Search anime",     body = AnimeSearchResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_anime_by_query(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
    info: web::Query<AnimeSearchParams>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::get_anime_by_query_service(config.clone(), client.get_ref(), info.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/search",
    tag = "Anime",
    responses(
        (status = 200, description = "Search anime",     body = AnimeSearchResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn search_anime_in_local_db(
    client: web::Data<Client>,
    info: web::Query<AnimeSearchParams>,
) -> Result<HttpResponse, ApiError> {
    let results = anime_service::search_anime_in_local_db_service(client.get_ref(), info.into_inner()).await?;

    Ok(HttpResponse::Ok().json(results))
}


// Fetch from local db
#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "Anime",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u64>, Query, description = "Items per page (default: 25, max: 100)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by: score, rank, popularity, year, episodes"),
        ("order" = Option<String>, Query, description = "Sort order: asc, desc"),
    ),
    responses(
        (status = 200, description = "List of anime", body = AnimeSearchResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_anime_list(
    client: web::Data<Client>,
    info: web::Query<AnimeListParams>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::get_anime_list_service(client.get_ref(), info.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}


#[utoipa::path(
    post,
    path = "/api/anime",
    tag = "Anime",
    request_body = CreateAnimeRequest,
    responses(
        (status = 201, description = "Anime created",       body = AnimeByIdResponse),
        (status = 409, description = "Anime already exists"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn create_anime(
    client: web::Data<Client>,
    body: web::Json<CreateAnimeRequest>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::create_anime_service(client.get_ref(), body.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

#[utoipa::path(
    put,
    path = "/api/anime/{id}",
    tag = "Anime",
    request_body = UpdateAnimeRequest,
    responses(
        (status = 200, description = "Anime updated",   body = AnimeByIdResponse),
        (status = 404, description = "Anime not found"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn update_anime(
    client: web::Data<Client>,
    path: web::Path<i32>,
    body: web::Json<UpdateAnimeRequest>,
) -> Result<HttpResponse, ApiError> {
    let result = anime_service::update_anime_service(client.get_ref(), path.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

//Characters
