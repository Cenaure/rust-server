use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub groups: Vec<Group>,
    pub last_login: Option<String>
}

// Return Dto
#[derive(Deserialize, Serialize)]
pub struct UserDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, 
    pub username: String,
    pub email: String,
    pub groups: Vec<Group>,
    pub last_login: Option<String>
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