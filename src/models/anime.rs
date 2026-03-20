use crate::jikan_integration::common::enums::anime::{AnimeFilter, AnimeRating, AnimeType};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TopAnimeParams {
    pub r#type: Option<AnimeType>,
    pub filter: Option<AnimeFilter>,
    pub rating: Option<AnimeRating>,
    pub sfw: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}