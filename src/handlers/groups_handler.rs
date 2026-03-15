use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::models::GroupDTO;
use actix_web::{get, post, web, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

pub const GROUPS_COLL_NAME: &str = "groups";

#[get("/")]
pub async fn list_groups(client: web::Data<Client>) -> Result<HttpResponse,ApiError>  {
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

#[post("/")]
pub async fn add_group(client: web::Data<Client>) -> Result<HttpResponse,ApiError> {
    todo!()
}