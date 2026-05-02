use crate::jikan_integration::common::structs::common::{CommonMalResponse, Images, Pagination};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct AnimeStruct {
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

    #[serde(rename = "producer_ids", default)]
    pub producers: Option<Vec<u32>>,

    pub studios: Option<Vec<CommonMalResponse>>,
    pub genres: Option<Vec<CommonMalResponse>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct AnimePopulated {
    #[serde(flatten)]
    pub anime: AnimeStruct,
    pub producers: Vec<CommonMalResponse>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct AnimeTrailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct AnimeTitles {
    pub r#type: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct AnimeByIdResponse {
    pub data: AnimePopulated,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeSearchResponse {
    pub pagination: Pagination,
    pub data: Vec<AnimePopulated>,
}