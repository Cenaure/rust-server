use crate::jikan_integration::common::structs::common::CommonMalResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Anime Struct, only needed fields
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeStruct {
    pub mal_id: u32,
    pub url: Option<String>,
    pub images: Option<AnimeImages>,
    pub trailer: Option<AnimeTrailer>,
    //pub approved
    pub titles: Option<Vec<AnimeTitles>>,
    pub r#type: Option<String>,
    //pub source
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    //pub aired
    //pub duration
    pub rating: Option<String>,

    // Score related
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,

    //pub members
    //pub favourites

    pub synopsis: Option<String>,
    pub background: Option<String>,
    //pub season
    pub year: Option<u16>,
    //pub broadcast
    pub producers: Option<Vec<CommonMalResponse>>,
    //pub licensors
    pub studios: Option<Vec<CommonMalResponse>>,
    pub genres: Option<Vec<CommonMalResponse>>,
    //pub explicit genres
    //pub themes
    //pub demographics
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeImages {
    pub webp: WebpImage
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct WebpImage {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeTrailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeTitles {
    pub r#type: String,
    pub title: String,
}