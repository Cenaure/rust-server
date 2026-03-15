use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct GroupDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub permissions: Vec<String>,
}