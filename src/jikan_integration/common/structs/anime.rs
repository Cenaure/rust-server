use crate::jikan_integration::common::structs::common::CommonMalResponse;

// Anime Struct, only needed fields
pub struct AnimeStruct {
    pub mal_id: u32,
    pub url: String,
    pub images: AnimeImages,
    pub trailer: AnimeTrailer,
    //pub approved
    pub titles: Vec<AnimeTitles>,
    pub r#type: Option<String>,
    //pub source
    pub episodes: u32,
    pub status: Option<String>,
    pub airing: bool,
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

    pub synipsis: Option<String>,
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

pub struct AnimeImages {
    pub webp: WebpImage
}

pub struct WebpImage {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

pub struct AnimeTrailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

pub struct AnimeTitles {
    pub r#type: String,
    pub title: String,
}