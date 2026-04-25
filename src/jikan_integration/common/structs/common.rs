use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Pagination
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Pagination {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub current_page: i32,
    pub items: PaginationItems
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PaginationItems {
    pub count: i32,
    pub total: i32,
    pub per_page: i32,
}


// Common MAL response
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CommonMalResponse {
    pub mal_id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String
}