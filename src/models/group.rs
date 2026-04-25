use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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