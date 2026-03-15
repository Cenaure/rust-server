use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::models::{User, UserCreate, UserDTO, UserUpdate};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::options::ReturnDocument;
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
        groups: user_dto.groups.clone().unwrap_or(vec![]),
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

#[get("/{id}")]
pub async fn get_user(path: web::Path<String>, client: web::Data<Client>) -> Result<HttpResponse,ApiError>  {
    let collection: Collection<UserDTO> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let user_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid user ID".to_string()))?;

    let user = collection
        .find_one(doc! {"_id": user_id})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if user.is_none() {
        return Err(ApiError::NotFound("Users Not Found".to_string()));
    }

    Ok(HttpResponse::Ok().json(user))
}

#[patch("/{id}")]
pub async fn patch_user(path: web::Path<String>, client: web::Data<Client>, user_dto: web::Json<UserUpdate>) -> Result<HttpResponse,ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let user_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid user id".to_string()))?;

    let mut set_doc = Document::new();

    if let Some(username) = &user_dto.username {
        set_doc.insert("username", username);
    }
    if let Some(email) = &user_dto.email {
        set_doc.insert("email", email);
    }
    if let Some(groups) = &user_dto.groups {
        set_doc.insert(
            "groups",
            to_bson(groups).map_err(|e| ApiError::InternalServer(e.to_string()))?,
        );
    }

    if set_doc.is_empty() {
        return Err(ApiError::BadRequest("No fields to update".to_string()));
    }

    let updated = collection
        .find_one_and_update(
            doc! {"_id": user_id},
            doc! {"$set": set_doc},
        )
        .return_document(ReturnDocument::After)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .ok_or(ApiError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(UserDTO {
        id: updated.id,
        username: updated.username,
        email: updated.email,
        groups: updated.groups,
        last_login: updated.last_login,
    }))
}

#[delete("/{id}")]
pub async fn delete_user(path: web::Path<String>, client: web::Data<Client>) -> Result<HttpResponse,ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let user_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid user id".to_string()))?;

    let result = collection
        .delete_one(doc! {"_id": user_id})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if result.deleted_count == 0 {
        return Err(ApiError::NotFound("User not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}