use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Pagination
#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct Pagination {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub current_page: i32,
    pub items: PaginationItems
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct PaginationItems {
    pub count: i32,
    pub total: i32,
    pub per_page: i32,
}


// Common MAL response
#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct CommonMalResponse {
    pub mal_id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String
}

// Images
#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct Images {
    pub webp: WebpImage,
    pub jpg: JpgImage
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct WebpImage {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct JpgImage {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
}