use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::models::{Group, GroupCreate, GroupDTO, GroupUpdate};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::options::ReturnDocument;
use mongodb::{Client, Collection};

pub const GROUPS_COLL_NAME: &str = "groups";

#[get("/")]
pub async fn list_groups(client: web::Data<Client>) -> Result<HttpResponse, ApiError> {
    let collection: Collection<GroupDTO> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let groups: Vec<GroupDTO> = collection
        .find(doc! {})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .try_collect()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    Ok(HttpResponse::Ok().json(groups))
}

#[get("/{id}")]
pub async fn get_group(
    path: web::Path<String>,
    client: web::Data<Client>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<GroupDTO> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let group_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid group id".to_string()))?;

    let group = collection
        .find_one(doc! {"_id": group_id})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .ok_or(ApiError::NotFound("Group not found".to_string()))?;

    Ok(HttpResponse::Ok().json(group))
}

#[post("/")]
pub async fn add_group(
    client: web::Data<Client>,
    group_dto: web::Json<GroupCreate>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let new_group = Group {
        id: None,
        name: group_dto.name.clone(),
        permissions: group_dto.permissions.clone(),
    };

    let insert_result = collection
        .insert_one(&new_group)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let inserted_id = insert_result
        .inserted_id
        .as_object_id()
        .ok_or(ApiError::InternalServer("Failed to get inserted id".to_string()))?;

    Ok(HttpResponse::Created().json(GroupDTO {
        id: Some(inserted_id),
        name: new_group.name,
        permissions: new_group.permissions,
    }))
}

#[patch("/{id}")]
pub async fn patch_group(
    path: web::Path<String>,
    client: web::Data<Client>,
    group_dto: web::Json<GroupUpdate>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let group_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid group id".to_string()))?;

    let mut set_doc = Document::new();

    if let Some(name) = &group_dto.name {
        set_doc.insert("name", name);
    }
    if let Some(permissions) = &group_dto.permissions {
        set_doc.insert(
            "permissions",
            to_bson(permissions).map_err(|e| ApiError::InternalServer(e.to_string()))?,
        );
    }

    if set_doc.is_empty() {
        return Err(ApiError::BadRequest("No fields to update".to_string()));
    }

    let updated = collection
        .find_one_and_update(doc! {"_id": group_id}, doc! {"$set": set_doc})
        .return_document(ReturnDocument::After)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .ok_or(ApiError::NotFound("Group not found".to_string()))?;

    Ok(HttpResponse::Ok().json(GroupDTO {
        id: updated.id,
        name: updated.name,
        permissions: updated.permissions,
    }))
}

#[delete("/{id}")]
pub async fn delete_group(
    path: web::Path<String>,
    client: web::Data<Client>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let group_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid group id".to_string()))?;

    let result = collection
        .delete_one(doc! {"_id": group_id})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if result.deleted_count == 0 {
        return Err(ApiError::NotFound("Group not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}