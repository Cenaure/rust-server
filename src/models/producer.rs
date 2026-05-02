use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use crate::jikan_integration::common::structs::common::Pagination;
use crate::models::common::SortOrder;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProducerDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,
    pub mal_id: u32,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListProducersResponse {
    pub pagination: Pagination,
    pub data: Vec<ProducerDTO>,
}

// Query
#[derive(Debug, Deserialize, IntoParams)]
pub struct ProducerListParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub name: Option<String>,
    pub sort_by: Option<ProducerSortBy>,
    pub order: Option<SortOrder>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProducerSortBy {
    Name,
    MalId,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateProducerRequest {
    pub name: Option<String>,
    pub url: Option<String>,
}