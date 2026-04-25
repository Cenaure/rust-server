use crate::jikan_integration::common::enums::anime::{AnimeFilter, AnimeRating, AnimeType};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct TopAnimeParams {
    pub r#type: Option<AnimeType>,
    pub filter: Option<AnimeFilter>,
    pub rating: Option<AnimeRating>,
    pub sfw: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}