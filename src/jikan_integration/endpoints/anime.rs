use actix_web::web;
use log::info;
use crate::errors::ApiError;
use crate::jikan_integration::common::structs::anime::{AnimeByIdResponse, AnimeStruct};
use crate::utils::app_config::AppConfig;
use reqwest::StatusCode;
use crate::jikan_integration::common::structs::character::{AnimeCharactersResponse};

//anime
pub async fn get_anime_by_id(
    config: &AppConfig,
    id: i32
) -> Result<AnimeByIdResponse, ApiError> {

    let url = format!("{}/anime/{}", config.jikan_api_url, id);

    info!("{}", url);
    let response = config.http_client
        .get(url)
        .send()
        .await
        .map_err(|e| ApiError::BadGateway(format!("Jikan request failed: {e}")))?;

    match response.status() {
        StatusCode::OK => {
            response.json::<AnimeByIdResponse>().await
                .map_err(|e| ApiError::InternalServer(format!("Failed to parse Jikan response: {e}")))
        }
        StatusCode::NOT_FOUND => {
            Err(ApiError::NotFound(format!("Anime ID {id} not found in Jikan")))
        }
        StatusCode::TOO_MANY_REQUESTS => {
            Err(ApiError::TooManyRequests("Jikan rate limit hit".to_string()))
        }
        other => {
            Err(ApiError::BadGateway(format!("Jikan returned status: {other}")))
        }
    }
}

//characters
pub async fn get_anime_characters(
    config: &AppConfig,
    id: i32
) -> Result<AnimeCharactersResponse, ApiError> {

    let url = format!("{}/anime/{}/characters", config.jikan_api_url, id);

    info!("{}", url);
    let response = config.http_client
        .get(url)
        .send()
        .await
        .map_err(|e| ApiError::BadGateway(format!("Jikan request failed: {e}")))?;

    match response.status() {
        StatusCode::OK => {
            response.json::<AnimeCharactersResponse>().await
                .map_err(|e| ApiError::InternalServer(format!("Failed to parse Jikan response: {e}")))
        }
        StatusCode::NOT_FOUND => {
            Err(ApiError::NotFound(format!("Anime ID {id} not found in Jikan")))
        }
        StatusCode::TOO_MANY_REQUESTS => {
            Err(ApiError::TooManyRequests("Jikan rate limit hit".to_string()))
        }
        other => {
            Err(ApiError::BadGateway(format!("Jikan returned status: {other}")))
        }
    }
}