use crate::jikan_integration::common::structs::anime::{AnimePopulated, AnimeStruct};
use crate::jikan_integration::common::structs::common::Pagination;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnimeTopJikanResponse {
    pub pagination: Pagination,
    pub data: Vec<AnimePopulated>,
}