use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::models::common::SortOrder;
use crate::models::{Group, GroupCreate, GroupDTO, GroupListParams, GroupSortBy, GroupUpdate};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::options::{FindOptions, ReturnDocument};
use mongodb::{Client, Collection};

pub const GROUPS_COLL_NAME: &str = "groups";

pub async fn list_groups_service(client: &Client, info: &GroupListParams) -> Result<serde_json::Value, ApiError> {
    let collection: Collection<GroupDTO> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let per_page = info.limit.unwrap_or(10) as i64;
    let current_page = info.page.unwrap_or(1) as i64;
    let skip = ((current_page - 1) * per_page) as u64;

    let filter = build_filter(info);
    let sort_doc = build_sort(info);

    let total_count = collection
        .count_documents(filter.clone())
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))? as i64;

    let find_options = FindOptions::builder()
        .limit(per_page)
        .skip(skip)
        .sort(sort_doc)
        .build();

    let groups: Vec<GroupDTO> = collection
        .find(filter)
        .with_options(find_options)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .try_collect()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let last_visible_page = (total_count as f64 / per_page as f64).ceil() as i64;
    let has_next_page = current_page < last_visible_page;

    Ok(serde_json::json!({
        "pagination": {
            "last_visible_page": last_visible_page,
            "has_next_page": has_next_page,
            "current_page": current_page,
            "items": {
                "count": groups.len(),
                "total": total_count,
                "per_page": per_page,
            }
        },
        "data": groups
    }))
}

pub async fn get_group_service(client: &Client, id: String) -> Result<GroupDTO, ApiError> {
    let collection: Collection<GroupDTO> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);
    let group_id = ObjectId::parse_str(id)
        .map_err(|_| ApiError::BadRequest("Invalid group id".to_string()))?;

    collection
        .find_one(doc! {"_id": group_id})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .ok_or(ApiError::NotFound("Group not found".to_string()))
}

pub async fn get_groups_by_ids_service(client: &Client, ids: &[ObjectId]) -> Result<Vec<Group>, ApiError> {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);
    let filter = doc! { "_id": { "$in": ids } };

    collection
        .find(filter)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .try_collect()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))
}

pub async fn add_group_service(client: &Client, group_dto: GroupCreate) -> Result<GroupDTO, ApiError> {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let new_group = Group {
        id: None,
        name: group_dto.name,
        permissions: group_dto.permissions,
    };

    let insert_result = collection
        .insert_one(&new_group)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let inserted_id = insert_result
        .inserted_id
        .as_object_id()
        .ok_or(ApiError::InternalServer("Failed to get inserted id".to_string()))?;

    Ok(GroupDTO {
        id: Some(inserted_id),
        name: new_group.name,
        permissions: new_group.permissions,
    })
}

pub async fn patch_group_service(
    client: &Client,
    id: String,
    group_dto: GroupUpdate,
) -> Result<GroupDTO, ApiError> {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let group_id = ObjectId::parse_str(id)
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

    Ok(GroupDTO {
        id: updated.id,
        name: updated.name,
        permissions: updated.permissions,
    })
}

pub async fn delete_group_service(client: &Client, id: String) -> Result<(), ApiError> {
    let collection: Collection<Group> = client.database(DB_NAME).collection(GROUPS_COLL_NAME);

    let group_id = ObjectId::parse_str(id)
        .map_err(|_| ApiError::BadRequest("Invalid group id".to_string()))?;

    let result = collection
        .delete_one(doc! {"_id": group_id})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if result.deleted_count == 0 {
        return Err(ApiError::NotFound("Group not found".to_string()));
    }

    Ok(())
}

fn build_filter(params: &GroupListParams) -> Document {
    let mut filter = doc! {};

    if let Some(ref name) = params.name {
        if !name.trim().is_empty() {
            filter.insert(
                "name",
                doc! {
                    "$regex": name.trim(),
                    "$options": "i"
                },
            );
        }
    }

    filter
}

fn build_sort(params: &GroupListParams) -> Document {
    let direction: i32 = match params.order {
        Some(SortOrder::Desc) => -1,
        _ => 1,
    };

    let field = match params.sort_by {
        Some(GroupSortBy::Name) => "name",
        None => "_id",
    };

    doc! { field: direction }
}
