use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::models::{User, UserDTO};
use actix_web::{get, post, web, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

pub const USERS_COLL_NAME: &str = "users";

#[get("/users")]
pub async fn list_users(client: web::Data<Client>) -> Result<HttpResponse,ApiError>  {
    let collection: Collection<UserDTO> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let users: Vec<UserDTO> = collection
        .find(doc! {})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .try_collect()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if users.is_empty() {
        return Err(ApiError::NotFound("Users Not Found".to_string()));
    }

    Ok(HttpResponse::Ok().json(users))
}

#[post("/users")]
pub async fn add_user(client: web::Data<Client>, form: web::Json<UserDTO>) -> Result<HttpResponse,ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    todo!()
}