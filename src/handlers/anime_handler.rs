use crate::errors::ApiError;
use crate::jikan_integration::common::structs::random::AnimeRandomJikanResponse;
use crate::jikan_integration::common::structs::top::AnimeTopJikanResponse;
use crate::jikan_integration::endpoints::random::get_random_anime;
use crate::jikan_integration::endpoints::top::get_top_anime;
use crate::models::{AnimeSearchParams, CreateAnimeRequest, TopAnimeParams, UpdateAnimeRequest};
use crate::utils::app_config::AppConfig;
use actix_web::{get, post, put, web, HttpResponse};
use futures::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use utoipa::openapi::Info;
use crate::handlers::DB_NAME;
use crate::jikan_integration::common::structs::anime::{AnimeByIdResponse, AnimeSearchResponse, AnimeStruct};
use crate::jikan_integration::common::structs::character::{AnimeCharactersResponse};
use crate::jikan_integration::endpoints::anime::{get_anime_by_id, get_anime_characters, search_anime};

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
    if let Err(e) = collection.insert_one(&result.data).await {
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
    if let Err(e) = collection.insert_one(&result.data).await {
        log::error!("Failed to cache anime {}: {}", id, e);
    }
    Ok(HttpResponse::Ok().json(result))
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
    if let Err(e) = collection.insert_many(&result.data).await {
        log::error!("Failed to cache anime: {}", e);
    }

    Ok(HttpResponse::Ok().json(result))
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