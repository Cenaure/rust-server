use crate::errors::ApiError;
use crate::models::common::SortOrder;
use crate::models::{Group, GroupCreate, GroupDTO, GroupListParams, GroupSortBy, GroupUpdate};
use crate::services::groups_service;
use actix_web::{web, HttpResponse};
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

#[utoipa::path(
    get,
    path = "/api/groups/",
    tag = "Groups",
    params(
        ("name" = Option<String>, Query, description = "Filter by group name"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page"),
        ("sort_by" = Option<GroupSortBy>, Query, description = "Field to sort by"),
        ("order" = Option<SortOrder>, Query, description = "Sort direction")
    ),
    responses(
        (status = 200, description = "List groups with pagination", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_groups(
    client: web::Data<Client>,
    info: web::Query<GroupListParams>,
) -> Result<HttpResponse, ApiError> {
    let response = groups_service::list_groups_service(client.get_ref(), &info).await?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/groups/{id}",
    tag = "Groups",
    params(
        ("id" = String, Path, description = "Group id")
    ),
    responses(
        (status = 200, description = "Group details", body = GroupDTO),
        (status = 400, description = "Invalid group id"),
        (status = 404, description = "Group not found")
    )
)]
pub async fn get_group(
    path: web::Path<String>,
    client: web::Data<Client>,
) -> Result<HttpResponse, ApiError> {
    let group = groups_service::get_group_service(client.get_ref(), path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(group))
}

pub async fn get_groups_by_ids(
    client: &Client,
    ids: &[ObjectId],
) -> Result<Vec<Group>, ApiError> {
    groups_service::get_groups_by_ids_service(client, ids).await
}

#[utoipa::path(
    post,
    path = "/api/groups/",
    tag = "Groups",
    request_body = GroupCreate,
    responses(
        (status = 201, description = "Group created", body = GroupDTO),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_group(
    client: web::Data<Client>,
    group_dto: web::Json<GroupCreate>,
) -> Result<HttpResponse, ApiError> {
    let new_group = groups_service::add_group_service(client.get_ref(), group_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_group))
}

#[utoipa::path(
    patch,
    path = "/api/groups/{id}",
    tag = "Groups",
    request_body = GroupUpdate,
    params(
        ("id" = String, Path, description = "Group id")
    ),
    responses(
        (status = 200, description = "Updated group", body = GroupDTO),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Group not found")
    )
)]
pub async fn patch_group(
    path: web::Path<String>,
    client: web::Data<Client>,
    group_dto: web::Json<GroupUpdate>,
) -> Result<HttpResponse, ApiError> {
    let updated = groups_service::patch_group_service(client.get_ref(), path.into_inner(), group_dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}
#[utoipa::path(
    delete,
    path = "/api/groups/{id}",
    tag = "Groups",
    params(
        ("id" = String, Path, description = "Group id")
    ),
    responses(
        (status = 204, description = "Group deleted"),
        (status = 400, description = "Invalid group id"),
        (status = 404, description = "Group not found")
    )
)]
pub async fn delete_group(
    path: web::Path<String>,
    client: web::Data<Client>,
) -> Result<HttpResponse, ApiError> {
    groups_service::delete_group_service(client.get_ref(), path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}