use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::models::{User, UserCreate, UserDTO};
use actix_web::{get, post, web, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

pub const USERS_COLL_NAME: &str = "users";

#[get("/")]
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

#[post("/")]
pub async fn add_user(client: web::Data<Client>, user_dto: web::Json<UserCreate>) -> Result<HttpResponse,ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let new_user = User {
        id: None,
        username: user_dto.username.clone(),
        email: user_dto.email.clone(),
        password: bcrypt::hash(&user_dto.password, 12)
            .map_err(|e| ApiError::InternalServer(e.to_string()))?,
        groups: user_dto.groups.clone().unwrap_or(vec![]), // TODO idk
        last_login: None,
    };

    let insert_result = collection
        .insert_one(&new_user)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let inserted_id = insert_result
        .inserted_id
        .as_object_id()
        .ok_or(ApiError::InternalServer("Failed to get inserted id".to_string()))?;

    Ok(
        HttpResponse::Created().json(UserDTO {
            id: Some(inserted_id),
            username: new_user.username,
            email: new_user.email,
            groups: new_user.groups,
            last_login: new_user.last_login,
        })
    )
}