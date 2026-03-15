use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct GroupCreate {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GroupUpdate {
    pub name: Option<String>,
    pub permissions: Option<Vec<String>>,
}