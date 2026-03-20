use serde::{Deserialize, Serialize};

// Anime Type
#[derive(Serialize, Deserialize)]
pub enum AnimeType {
    TV,
    MOVIE,
    OVA,
    SPECIAL,
    ONA,
    MUSIC,
    CM,
    PV,
    TvSpecial
}

impl AnimeType {
    pub fn to_string(&self) -> &str {
        match self {
            AnimeType::TV => "tv",
            AnimeType::MOVIE => "movie",
            AnimeType::OVA => "ova",
            AnimeType::SPECIAL => "special",
            AnimeType::ONA => "ona",
            AnimeType::MUSIC => "music",
            AnimeType::CM => "cm",
            AnimeType::PV => "pv",
            AnimeType::TvSpecial => "tv_special"
        }
    }
}


// Anime Status
#[derive(Serialize, Deserialize)]
pub enum AnimeStatus {
    AIRING,
    COMPLETE,
    UPCOMING
}

impl AnimeStatus {
    pub fn to_string(&self) -> &str {
        match self {
            AnimeStatus::AIRING => "airing",
            AnimeStatus::COMPLETE => "complete",
            AnimeStatus::UPCOMING => "upcoming",
        }
    }
}


// Anime Filter
#[derive(Serialize, Deserialize)]
pub enum AnimeFilter {
    AIRING,
    UPCOMING,
    POPULARITY,
    FAVOURITE
}

impl AnimeFilter {
    pub fn to_string(&self) -> &str {
        match self {
            AnimeFilter::AIRING => "airing",
            AnimeFilter::UPCOMING => "upcoming",
            AnimeFilter::POPULARITY => "bypopularity",
            AnimeFilter::FAVOURITE => "favourite"
        }
    }
}


// Anime Rating
#[derive(Serialize, Deserialize)]
pub enum AnimeRating {
    G,
    PG,
    PG13,
    R17,
    R,
    Rx
}

impl AnimeRating {
    pub fn to_string(&self) -> &str {
        match self {
            AnimeRating::G => "g",
            AnimeRating::PG => "pg",
            AnimeRating::PG13 => "pg13",
            AnimeRating::R17 => "r17",
            AnimeRating::R => "r",
            AnimeRating::Rx => "rx"
        }
    }
}