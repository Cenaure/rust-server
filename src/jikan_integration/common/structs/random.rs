use crate::jikan_integration::common::structs::anime::AnimeStruct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeRandomJikanResponse {
    pub data: AnimeStruct
}