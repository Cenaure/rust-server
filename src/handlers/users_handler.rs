use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::models::{User, UserCreate, UserDTO, UserListParams, UserSortBy, UserUpdate};
use actix_web::{web, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOptions, ReturnDocument};
use mongodb::{Client, Collection};
use crate::handlers::groups_handler::get_groups_by_ids;
use crate::models::common::SortOrder;

pub const USERS_COLL_NAME: &str = "users";

#[utoipa::path(
    get,
    path = "/api/users/",
    tag = "Users",
    params(UserListParams),
    responses(
        (status = 200, description = "List users"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_users(
    client: web::Data<Client>,
    info: web::Query<UserListParams>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let per_page = info.limit.unwrap_or(10) as i64;
    let current_page = info.page.unwrap_or(1) as i64;
    let skip = ((current_page - 1) * per_page) as u64;

    let filter = build_filter(&info);

    let sort_doc = build_sort(&info);

    let total_count = collection
        .count_documents(filter.clone())
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))? as i64;

    let find_options = FindOptions::builder()
        .limit(per_page)
        .skip(skip)
        .sort(sort_doc)
        .build();

    let users: Vec<User> = collection
        .find(filter)
        .with_options(find_options)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .try_collect()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let last_visible_page = (total_count as f64 / per_page as f64).ceil() as i64;
    let has_next_page = current_page < last_visible_page;

    let response = serde_json::json!({
        "pagination": {
            "last_visible_page": last_visible_page,
            "has_next_page": has_next_page,
            "current_page": current_page,
            "items": {
                "count": users.len(),
                "total": total_count,
                "per_page": per_page,
            }
        },
        "data": users
    });

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/api/users/",
    tag = "Users",
    request_body = UserCreate,
    responses(
        (status = 201, description = "User created", body = UserDTO),
        (status = 500, description = "Internal server error")
    )
)]
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

    let groups = get_groups_by_ids(&client, &new_user.groups).await?;

    Ok(
        HttpResponse::Created().json(UserDTO {
            id: Some(inserted_id),
            username: new_user.username,
            email: new_user.email,
            groups,
            last_login: new_user.last_login,
        })
    )
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User details", body = UserDTO),
        (status = 400, description = "Invalid user ID"),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user(path: web::Path<String>, client: web::Data<Client>) -> Result<HttpResponse,ApiError>  {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

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

#[utoipa::path(
    patch,
    path = "/api/users/{id}",
    tag = "Users",
    request_body = UserUpdate,
    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "Updated user", body = UserDTO),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found")
    )
)]
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
        set_doc.insert("groups", groups);
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

    let groups = get_groups_by_ids(&client, &updated.groups).await?;

    Ok(HttpResponse::Ok().json(UserDTO {
        id: updated.id,
        username: updated.username,
        email: updated.email,
        groups,
        last_login: updated.last_login,
    }))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "Users",
    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 400, description = "Invalid user id"),
        (status = 404, description = "User not found")
    )
)]
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


//

fn build_filter(params: &UserListParams) -> Document {
    let mut filter = doc! {};

    if let Some(ref email) = params.email {
        if !email.trim().is_empty() {
            filter.insert(
                "email",
                doc! {
                    "$regex": email.trim(),
                    "$options": "i"   // case-insensitive
                },
            );
        }
    }

    filter
}

fn build_sort(params: &UserListParams) -> Document {
    let direction: i32 = match params.order {
        Some(SortOrder::Desc) => -1,
        _ => 1,
    };

    let field = match params.sort_by {
        Some(UserSortBy::Email) => "email",
        Some(UserSortBy::LastLogin) => "last_login",
        Some(UserSortBy::Username) => "username",
        None => "_id",
    };

    doc! { field: direction }
}