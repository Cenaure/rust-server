use futures::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOptions, ReplaceOptions, ReturnDocument};
use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::jikan_integration::common::structs::anime::AnimeSearchResponse;
use crate::jikan_integration::common::structs::common::{CommonMalResponse, Pagination, PaginationItems};
use crate::models::common::SortOrder;
use crate::models::producer::{ListProducersResponse, ProducerDTO, ProducerListParams, ProducerSortBy, UpdateProducerRequest};

pub const PRODUCERS_COLL_NAME: &str = "producers";

pub(crate) fn collection(client: &Client) -> Collection<ProducerDTO> {
    client.database(DB_NAME).collection(PRODUCERS_COLL_NAME)
}

//helpers
fn build_filter(params: &ProducerListParams) -> Document {
    let mut filter = doc! {};
    if let Some(name) = &params.name {
        filter.insert("name", doc! { "$regex": name, "$options": "i" });
    }
    filter
}

fn build_sort(params: &ProducerListParams) -> Document {
    let order = match params.order {
        Some(SortOrder::Desc) => -1,
        _ => 1,
    };
    match params.sort_by {
        Some(ProducerSortBy::MalId) => doc! { "mal_id": order },
        _ => doc! { "name": order },
    }
}

// cache
pub async fn cache_producers(
    collection: &Collection<ProducerDTO>,
    producers: &[&CommonMalResponse],
) {
    for p in producers {
        let filter = doc! {
            "mal_id": p.mal_id,
            "manually_edited": { "$ne": true }
        };
        let dto = ProducerDTO {
            id: None,
            mal_id: p.mal_id,
            name: p.name.clone(),
            url: Option::from(p.url.clone()),
        };
        let options = ReplaceOptions::builder().upsert(true).build();
        if let Err(e) = collection.replace_one(filter, &dto).with_options(options).await {
            log::warn!("Failed to cache producer {} ({}): {}", p.name, p.mal_id, e);
        }
    }
}

//

pub async fn list_producers_service(
    client: &Client,
    params: &ProducerListParams,
) -> Result<ListProducersResponse, ApiError> {
    let coll = collection(client);
    let per_page = params.limit.unwrap_or(10) as i64;
    let current_page = params.page.unwrap_or(1) as i64;
    let skip = ((current_page - 1) * per_page) as u64;

    let filter = build_filter(params);
    let sort_doc = build_sort(params);

    let total_count = coll
        .count_documents(filter.clone())
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))? as i64;

    let find_options = FindOptions::builder()
        .limit(per_page)
        .skip(skip)
        .sort(sort_doc)
        .build();

    let producers: Vec<ProducerDTO> = coll
        .find(filter)
        .with_options(find_options)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .try_collect()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let last_visible_page = (total_count as f64 / per_page as f64).ceil() as i32;

    Ok(ListProducersResponse {
        pagination: Pagination {
            last_visible_page,
            has_next_page: current_page < last_visible_page as i64,
            current_page: current_page as i32,
            items: PaginationItems {
                count: producers.len() as i32,
                total: total_count as i32,
                per_page: per_page as i32,
            },
        },
        data: producers,
    })
}

pub async fn get_producer_by_mal_id_service(
    client: &Client,
    mal_id: i64,
) -> Result<ProducerDTO, ApiError> {
    let coll = collection(client);
    let filter = doc! { "mal_id": mal_id };

    coll.find_one(filter)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound(format!("Producer with mal_id {} not found", mal_id)))
}

pub async fn update_producer_service(
    client: &Client,
    mal_id: u32,
    body: UpdateProducerRequest,
) -> Result<serde_json::Value, ApiError> {
    let coll = collection(client);
    let mut set_doc = doc! { "manually_edited": true };

    macro_rules! set_if_some {
        ($field:expr, $key:expr) => {
            if let Some(val) = $field {
                set_doc.insert($key, mongodb::bson::to_bson(&val)
                    .map_err(|e| ApiError::InternalServer(e.to_string()))?);
            }
        };
    }

    set_if_some!(body.name, "name");
    set_if_some!(body.url, "url");

    let result = coll
        .find_one_and_update(
            doc! { "mal_id": mal_id },
            doc! { "$set": set_doc },
        )
        .return_document(ReturnDocument::After)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    match result {
        Some(updated) => Ok(serde_json::json!({ "data": updated })),
        None => Err(ApiError::NotFound(format!("Producer {} not found", mal_id))),
    }
}