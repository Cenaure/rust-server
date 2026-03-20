use crate::jikan_integration::common::structs::anime::AnimeStruct;
use crate::jikan_integration::common::structs::common::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeTopJikanResponse {
    pagination: Pagination,
    data: Vec<AnimeStruct>,
}