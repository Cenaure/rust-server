use crate::errors::ApiError;
use crate::jikan_integration::common::structs::random::AnimeRandomJikanResponse;
use crate::jikan_integration::common::structs::top::AnimeTopJikanResponse;
use crate::jikan_integration::endpoints::random::get_random_anime;
use crate::jikan_integration::endpoints::top::get_top_anime;
use crate::models::{AnimeListParams, AnimeListSortBy, AnimeSearchParams, CreateAnimeRequest, TopAnimeParams, UpdateAnimeRequest};
use crate::utils::app_config::AppConfig;
use actix_web::{get, post, put, web, HttpResponse};
use futures::TryStreamExt;
use log::info;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use crate::handlers::DB_NAME;
use crate::jikan_integration::common::structs::anime::{AnimeByIdResponse, AnimeSearchResponse, AnimeStruct};
use crate::jikan_integration::common::structs::character::{AnimeCharactersResponse};
use crate::jikan_integration::common::structs::common::{Pagination, PaginationItems};
use crate::jikan_integration::endpoints::anime::{get_anime_by_id, get_anime_characters, search_anime};
use crate::models::common::SortOrder;

pub const ANIME_COLL_NAME: &str = "anime";

/// Upsert a slice of anime into the local DB.
/// Errors are logged but never bubbled up - caching is best-effort.
async fn cache_anime_list(collection: &Collection<AnimeStruct>, list: &[AnimeStruct]) {
    for anime in list {
        let filter = doc! { "mal_id": anime.mal_id };
        let options = mongodb::options::ReplaceOptions::builder()
            .upsert(true)
            .build();
        if let Err(e) = collection.replace_one(filter, anime).with_options(options).await {
            log::warn!("Failed to cache anime {}: {}", anime.mal_id, e);
        }
    }
}

// Endpoints
#[utoipa::path(
    get,
    path = "/api/anime/top",
    tag = "Anime",
    params(TopAnimeParams),
    responses(
        (status = 200, description = "Top anime list",  body = AnimeTopJikanResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_top(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
    params: web::Query<TopAnimeParams>,
) -> Result<HttpResponse, ApiError> {
    let params = params.into_inner();

    let result = get_top_anime(
        config,
        params.r#type,
        params.filter,
        params.rating,
        params.sfw,
        params.page,
        params.limit,
    )
        .await?;

    // Fire-and-forget: store every anime we just fetched so future
    // /api/anime/{id} requests can be served from the local DB.
    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);
    let list = result.data.clone();
    actix_web::rt::spawn(async move {
        cache_anime_list(&collection, &list).await;
    });

    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/random",
    tag = "Anime",
    responses(
        (status = 200, description = "Random anime", body = AnimeRandomJikanResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_random(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    // Try to serve a random document straight from the local DB.
    let pipeline = vec![doc! { "$sample": { "size": 1 } }];
    let mut cursor = collection
        .aggregate(pipeline)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if let Some(doc) = cursor.try_next().await.map_err(|e| ApiError::InternalServer(e.to_string()))? {
        let anime: AnimeStruct = mongodb::bson::from_document(doc)
            .map_err(|e| ApiError::InternalServer(e.to_string()))?;
        return Ok(HttpResponse::Ok().json(serde_json::json!({ "data": anime })));
    }

    // DB is empty — fall back to Jikan and cache the result.
    let result = get_random_anime(config).await?;
    let filter = doc! { "mal_id": result.data.mal_id };
    let options = mongodb::options::ReplaceOptions::builder()
        .upsert(true)
        .build();
    if let Err(e) = collection.replace_one(filter, &result.data).with_options(options).await {
        log::warn!("Failed to cache random anime: {}", e);
    }
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/{id}",
    tag = "Anime",
    responses(
        (status = 200, description = "Anime by id",     body = AnimeByIdResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_by_id(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    if let Some(anime) = collection
        .find_one(doc! { "mal_id": id })
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
    {
        return Ok(HttpResponse::Ok().json(serde_json::json!({ "data": anime })));
    }

    let result = get_anime_by_id(&config, id).await?;
    let filter = doc! { "mal_id": id };
    let options = mongodb::options::ReplaceOptions::builder()
        .upsert(true)
        .build();
    if let Err(e) = collection.replace_one(filter, &result.data).with_options(options).await {
        log::error!("Failed to cache anime {}: {}", id, e);
    }
    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/ids/{ids}",
    tag = "Anime",
    responses(
        (status = 200, description = "Anime by ids", body = AnimeSearchResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_anime_by_ids(
    client: web::Data<Client>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let ids_raw = path.into_inner();

    let ids: Vec<u32> = ids_raw
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();

    if ids.is_empty() {
        return Err(ApiError::BadRequest("No valid ids provided".to_string()));
    }

    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    let mut cursor = collection
        .find(doc! { "mal_id": { "$in": &ids } })
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let mut data: Vec<AnimeStruct> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        data.push(doc);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": data })))
}

#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "Anime",
    responses(
        (status = 200, description = "Search anime",     body = AnimeSearchResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn get_anime_by_query(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
    info: web::Query<AnimeSearchParams>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    let result = search_anime(&config, info.q.clone()).await?;
    cache_anime_list(&collection, &result.data).await;

    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path = "/api/anime/search",
    tag = "Anime",
    responses(
        (status = 200, description = "Search anime",     body = AnimeSearchResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error"),
    )
)]
pub async fn search_anime_in_local_db(
    config: web::Data<AppConfig>,
    client: web::Data<Client>,
    info: web::Query<AnimeSearchParams>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    let regex = doc! {
        "titles.english": {
            "$regex": &info.q,
            "$options": "i"
        }
    };

    let mut cursor = collection.find(regex).await?;

    let mut results: Vec<AnimeStruct> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        results.push(doc);
    }

    Ok(HttpResponse::Ok().json(results))
}


// Fetch from local db
#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "Anime",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<u64>, Query, description = "Items per page (default: 25, max: 100)"),
        ("sort_by" = Option<String>, Query, description = "Field to sort by: score, rank, popularity, year, episodes"),
        ("order" = Option<String>, Query, description = "Sort order: asc, desc"),
    ),
    responses(
        (status = 200, description = "List of anime", body = AnimeSearchResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_anime_list(
    client: web::Data<Client>,
    info: web::Query<AnimeListParams>,
) -> Result<HttpResponse, ApiError> {
    let limit = info.limit.min(100);
    let page = info.page.max(1);
    let skip = (page - 1) * limit;

    let sort_field = match info.sort_by.as_ref().unwrap_or(&AnimeListSortBy::Score) {
        AnimeListSortBy::Score      => "score",
        AnimeListSortBy::Rank       => "rank",
        AnimeListSortBy::Popularity => "popularity",
        AnimeListSortBy::Year       => "year",
        AnimeListSortBy::Episodes   => "episodes",
    };

    let sort_dir: i32 = match info.order.as_ref().unwrap_or(&SortOrder::Desc) {
        SortOrder::Asc  => 1,
        SortOrder::Desc => -1,
    };

    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    let total = collection
        .count_documents(doc! {})
        .await?;

    let find_options = FindOptions::builder()
        .sort(doc! { sort_field: sort_dir })
        .skip(skip as u64)
        .limit(limit as i64)
        .build();

    let mut cursor = collection.find(doc! {}).with_options(find_options).await?;

    let mut data: Vec<AnimeStruct> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        data.push(doc);
    }

    let last_visible_page = (total as f64 / limit as f64).ceil() as i32;

    Ok(HttpResponse::Ok().json(AnimeSearchResponse {
        pagination: Pagination {
            last_visible_page,
            has_next_page: page < last_visible_page,
            current_page: page,
            items: PaginationItems {
                count: data.len() as i32,
                total: total as i32,
                per_page: limit as i32,
            },
        },
        data,
    }))
}


#[utoipa::path(
    post,
    path = "/api/anime",
    tag = "Anime",
    request_body = CreateAnimeRequest,
    responses(
        (status = 201, description = "Anime created",       body = AnimeByIdResponse),
        (status = 409, description = "Anime already exists"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn create_anime(
    client: web::Data<Client>,
    body: web::Json<CreateAnimeRequest>,
) -> Result<HttpResponse, ApiError> {
    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    // Reject duplicates.
    if collection
        .find_one(doc! { "mal_id": body.mal_id })
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .is_some()
    {
        return Err(ApiError::Conflict(format!(
            "Anime with mal_id {} already exists",
            body.mal_id
        )));
    }

    let body = body.into_inner();
    let anime = AnimeStruct {
        mal_id: body.mal_id,
        url: body.url,
        images: body.images,
        trailer: body.trailer,
        titles: body.titles,
        r#type: body.r#type,
        episodes: body.episodes,
        status: body.status,
        airing: body.airing,
        rating: body.rating,
        score: body.score,
        scored_by: body.scored_by,
        rank: body.rank,
        popularity: body.popularity,
        synopsis: body.synopsis,
        background: body.background,
        year: body.year,
        producers: body.producers,
        studios: body.studios,
        genres: body.genres,
    };

    collection
        .insert_one(&anime)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    Ok(HttpResponse::Created().json(serde_json::json!({ "data": anime })))
}

#[utoipa::path(
    put,
    path = "/api/anime/{id}",
    tag = "Anime",
    request_body = UpdateAnimeRequest,
    responses(
        (status = 200, description = "Anime updated",   body = AnimeByIdResponse),
        (status = 404, description = "Anime not found"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn update_anime(
    client: web::Data<Client>,
    path: web::Path<i32>,
    body: web::Json<UpdateAnimeRequest>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let collection: Collection<AnimeStruct> =
        client.database(DB_NAME).collection(ANIME_COLL_NAME);

    // Build $set document from only the supplied (non-None) fields.
    let mut set_doc = doc! {};
    let body = body.into_inner();

    macro_rules! set_if_some {
        ($field:expr, $key:expr) => {
            if let Some(val) = $field {
                let bson_val = mongodb::bson::to_bson(&val)
                    .map_err(|e| ApiError::InternalServer(e.to_string()))?;
                set_doc.insert($key, bson_val);
            }
        };
    }

    set_if_some!(body.url,        "url");
    set_if_some!(body.titles,     "titles");
    set_if_some!(body.r#type,     "type");
    set_if_some!(body.episodes,   "episodes");
    set_if_some!(body.status,     "status");
    set_if_some!(body.airing,     "airing");
    set_if_some!(body.rating,     "rating");
    set_if_some!(body.score,      "score");
    set_if_some!(body.scored_by,  "scored_by");
    set_if_some!(body.rank,       "rank");
    set_if_some!(body.popularity, "popularity");
    set_if_some!(body.synopsis,   "synopsis");
    set_if_some!(body.background, "background");
    set_if_some!(body.year,       "year");
    set_if_some!(body.genres,     "genres");
    set_if_some!(body.studios,    "studios");
    set_if_some!(body.producers,  "producers");

    if set_doc.is_empty() {
        return Err(ApiError::BadRequest("No fields to update".into()));
    }

    let result = collection
        .find_one_and_update(
            doc! { "mal_id": id },
            doc! { "$set": set_doc },
        )
        .return_document(mongodb::options::ReturnDocument::After)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    match result {
        Some(updated) => Ok(HttpResponse::Ok().json(serde_json::json!({ "data": updated }))),
        None => Err(ApiError::NotFound(format!("Anime with id {} not found", id))),
    }
}

//Characters
#[utoipa::path(
    get,
    path = "/api/anime/{id}/characters",
    tag = "Anime",
    responses(
        (status = 200, description = "Characters by anime id", body = AnimeCharactersResponse),
        (status = 429, description = "Rate limit reached"),
        (status = 502, description = "Upstream error")
    )
)]
pub async fn get_characters(
    config: web::Data<AppConfig>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    let result = get_anime_characters(
        &config,
        id
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}