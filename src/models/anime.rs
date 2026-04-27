use crate::jikan_integration::common::enums::anime::{AnimeFilter, AnimeRating, AnimeType};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use crate::jikan_integration::common::structs::anime::{AnimeTitles, AnimeTrailer};
use crate::jikan_integration::common::structs::common::{CommonMalResponse, Images};

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct TopAnimeParams {
    pub r#type: Option<AnimeType>,
    pub filter: Option<AnimeFilter>,
    pub rating: Option<AnimeRating>,
    pub sfw: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateAnimeRequest {
    pub mal_id: u32,
    pub url: Option<String>,
    pub images: Option<Images>,
    pub trailer: Option<AnimeTrailer>,
    pub titles: Option<Vec<AnimeTitles>>,
    pub r#type: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub year: Option<u16>,
    pub producers: Option<Vec<CommonMalResponse>>,
    pub studios: Option<Vec<CommonMalResponse>>,
    pub genres: Option<Vec<CommonMalResponse>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UpdateAnimeRequest {
    pub url: Option<String>,
    pub titles: Option<Vec<AnimeTitles>>,
    pub r#type: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub year: Option<u16>,
    pub genres: Option<Vec<CommonMalResponse>>,
    pub studios: Option<Vec<CommonMalResponse>>,
    pub producers: Option<Vec<CommonMalResponse>>,
}

#[derive(Deserialize)]
pub struct AnimeSearchParams {
    pub q: String,
}