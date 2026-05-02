use actix_web::{web, HttpResponse};
use mongodb::Client;
use crate::errors::ApiError;
use crate::models::producer::{ListProducersResponse, ProducerDTO, ProducerListParams, UpdateProducerRequest};
use crate::services::{producers_service};

// Fetch from local db
#[utoipa::path(
    get,
    path = "/api/producers",
    tag = "Producers",
    params(
        ("sort_by" = Option<String>, Query, description = "Field to sort by: name, mal_id"),
        ("order" = Option<String>, Query, description = "Sort order: asc, desc"),
    ),
    responses(
        (status = 200, description = "List of anime", body = ListProducersResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_anime_list(
    client: web::Data<Client>,
    info: web::Query<ProducerListParams>,
) -> Result<HttpResponse, ApiError> {
    let result = producers_service::list_producers_service(client.get_ref(), &info.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/producers/{mal_id}",
    tag = "Producers",
    params(
        ("mal_id" = i64, Path, description = "The MAL ID of the producer"),
    ),
    responses(
        (status = 200, description = "Producer found", body = ProducerDTO),
        (status = 404, description = "Producer not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_producer_by_mal_id(
    client: web::Data<Client>,
    path: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let mal_id = path.into_inner();
    let result = producers_service::get_producer_by_mal_id_service(client.get_ref(), mal_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    patch,
    path = "/api/producers/{mal_id}",
    tag = "Producers",
    params(
        ("mal_id" = u32, Path, description = "The MAL ID of the producer to update"),
    ),
    request_body = UpdateProducerRequest,
    responses(
        (status = 200, description = "Producer updated successfully"),
        (status = 404, description = "Producer not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn update_producer(
    client: web::Data<Client>,
    path: web::Path<u32>,
    body: web::Json<UpdateProducerRequest>,
) -> Result<HttpResponse, ApiError> {
    let mal_id = path.into_inner();
    let result = producers_service::update_producer_service(client.get_ref(), mal_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}