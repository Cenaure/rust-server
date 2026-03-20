use crate::errors::ApiError;
use crate::jikan_integration::common::enums::anime::{AnimeFilter, AnimeRating, AnimeType};
use crate::jikan_integration::common::structs::top::AnimeTopJikanResponse;
use crate::utils::app_config::AppConfig;
use actix_web::web;
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct TopAnimeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<AnimeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<AnimeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating: Option<AnimeRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>
}

pub async fn get_top_anime(
    config: web::Data<AppConfig>,
    r#type: Option<AnimeType>,
    filter: Option<AnimeFilter>,
    rating: Option<AnimeRating>,
    sfw: Option<bool>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<AnimeTopJikanResponse, ApiError> {
    let query = TopAnimeQuery { r#type, filter, rating, sfw, page, limit };

    let response = config.http_client
        .get(format!("{}/top/anime/", config.jikan_api_url))
        .query(&query)
        .send()
        .await
        .map_err(|e| ApiError::BadGateway(format!("Jikan request failed: {e}")))?;

    match response.status() {
        StatusCode::OK => {
            response.json::<AnimeTopJikanResponse>().await
                .map_err(|e| ApiError::InternalServer(format!("Failed to parse response: {e}")))
        }
        StatusCode::TOO_MANY_REQUESTS => Err(ApiError::TooManyRequests("Jikan rate limit hit".to_string())),
        other => Err(ApiError::BadGateway(format!("Jikan returned {other}"))),
    }
}