use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::jikan_integration::common::structs::common::Images;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeCharactersResponse {
    pub data: Vec<AnimeCharacter>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeCharacter {
    pub character: Character,
    pub role: String,
    // pub voice_actors
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Character {
    pub mal_id: u32,
    pub images: Images,
    pub name: String,
    pub url: String,
}