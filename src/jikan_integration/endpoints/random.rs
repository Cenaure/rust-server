use crate::errors::ApiError;
use crate::jikan_integration::common::structs::random::AnimeRandomJikanResponse;
use crate::utils::app_config::AppConfig;
use actix_web::web;
use reqwest::StatusCode;

pub async fn get_random_anime(
    config: web::Data<AppConfig>,
) -> Result<AnimeRandomJikanResponse, ApiError> {

    let response = config.http_client
        .get(format!("{}/random/anime/", config.jikan_api_url))
        .send()
        .await
        .map_err(|e| ApiError::BadGateway(format!("Jikan request failed: {e}")))?;

    match response.status() {
        StatusCode::OK => {
            response.json::<AnimeRandomJikanResponse>().await
                .map_err(|e| ApiError::InternalServer(format!("Failed to parse response: {e}")))
        }
        StatusCode::TOO_MANY_REQUESTS => Err(ApiError::TooManyRequests("Jikan rate limit hit".to_string())),
        other => Err(ApiError::BadGateway(format!("Jikan returned {other}"))),
    }
}