use crate::models::Group;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub groups: Vec<Group>,
    pub last_login: Option<DateTime>
}

// User which returns to the client
#[derive(Deserialize, Serialize)]
pub struct UserDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, 
    pub username: String,
    pub email: String,
    pub groups: Vec<Group>,
    pub last_login: Option<DateTime>
}

//region: ---Auth
#[derive(Deserialize)]
pub struct UserSignIn {
    pub username_or_email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UserSignUp {
    pub username: String,
    pub email: String,
    pub password: String,
}
//endregion: ---Auth

//region: ---Users Handler
#[derive(Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub email: String,
    pub groups: Option<Vec<Group>>,
}

#[derive(Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub groups: Option<Vec<Group>>,
}
//endregion: ---Users Handler