use crate::jikan_integration::common::structs::anime::AnimeStruct;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeRandomJikanResponse {
    pub data: AnimeStruct
}