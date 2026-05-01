use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use crate::models::common::SortOrder;
use crate::models::UserSortBy;

#[derive(Serialize, Deserialize, ToSchema)]
#[derive(Clone)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub permissions: Vec<String>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GroupDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct GroupCreate {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GroupUpdate {
    pub name: Option<String>,
    pub permissions: Option<Vec<String>>,
}


// query
#[derive(Debug, Deserialize, IntoParams)]
pub struct GroupListParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub name: Option<String>,
    pub sort_by: Option<GroupSortBy>,
    pub order: Option<SortOrder>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum GroupSortBy {
    Name
}