use crate::errors::ApiError;
use crate::handlers::DB_NAME;
use crate::jikan_integration::common::structs::anime::{AnimePopulated, AnimeSearchResponse, AnimeStruct};
use crate::jikan_integration::common::structs::character::AnimeCharactersResponse;
use crate::jikan_integration::common::structs::common::{CommonMalResponse, Pagination, PaginationItems};
use crate::jikan_integration::common::structs::random::AnimeRandomJikanResponse;
use crate::jikan_integration::common::structs::top::AnimeTopJikanResponse;
use crate::jikan_integration::endpoints::anime::{get_anime_by_id, get_anime_characters, search_anime};
use crate::jikan_integration::endpoints::random::get_random_anime;
use crate::jikan_integration::endpoints::top::get_top_anime;
use crate::models::{AnimeListParams, AnimeListSortBy, AnimeSearchParams, CreateAnimeRequest, TopAnimeParams, UpdateAnimeRequest};
use crate::models::common::SortOrder;
use crate::utils::app_config::AppConfig;
use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::{Client, Collection};
use crate::models::producer::ProducerDTO;
use crate::services::producers_service;
use crate::services::producers_service::{cache_producers, PRODUCERS_COLL_NAME};

pub const ANIME_COLL_NAME: &str = "anime";

fn collection(client: &Client) -> Collection<AnimeStruct> {
    client.database(DB_NAME).collection(ANIME_COLL_NAME)
}

async fn cache_anime_list(
    anime_coll: &Collection<AnimeStruct>,
    producer_coll: &Collection<ProducerDTO>,
    list: &[AnimePopulated],
) {
    let producers: Vec<&CommonMalResponse> = {
        let mut seen = std::collections::HashSet::new();
        list.iter()
            .flat_map(|anime| anime.producers.iter())
            .filter(|p| seen.insert(p.mal_id))
            .collect()
    };

    for populated in list {
        let producer_ids: Vec<u32> = populated.producers
            .iter()
            .map(|p| p.mal_id)
            .collect();

        let anime = AnimeStruct {
            producers: Some(producer_ids),
            ..populated.anime.clone()
        };

        let filter = doc! { "mal_id": anime.mal_id };
        let options = mongodb::options::ReplaceOptions::builder().upsert(true).build();
        if let Err(e) = anime_coll.replace_one(filter, &anime).with_options(options).await {
            log::warn!("Failed to cache anime {}: {}", anime.mal_id, e);
        }
    }

    cache_producers(producer_coll, &producers).await;
}

pub async fn get_top_anime_service(
    config: web::Data<AppConfig>,
    client: &Client,
    params: TopAnimeParams,
) -> Result<AnimeTopJikanResponse, ApiError> {
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

    let anime_coll = collection(client);
    let producer_coll = producers_service::collection(client);
    let list: Vec<AnimePopulated> = result.data.iter().map(|raw| AnimePopulated {
        anime: AnimeStruct {
            producers: None,
            ..raw.anime.clone()
        },
        producers: raw.producers.clone(),
    }).collect();

    actix_web::rt::spawn(async move {
        cache_anime_list(&anime_coll, &producer_coll, &list).await;
    });

    Ok(result)
}

pub async fn get_random_anime_service(
    config: web::Data<AppConfig>,
    client: &Client,
) -> Result<serde_json::Value, ApiError> {
    let coll = collection(client);
    let pipeline = vec![doc! { "$sample": { "size": 1 } }];
    let mut cursor = coll
        .aggregate(pipeline)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if let Some(doc) = cursor
        .try_next()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
    {
        let anime: AnimeStruct =
            mongodb::bson::from_document(doc).map_err(|e| ApiError::InternalServer(e.to_string()))?;
        return Ok(serde_json::json!({ "data": anime }));
    }

    let result: AnimeRandomJikanResponse = get_random_anime(config).await?;
    let filter = doc! { "mal_id": result.data.mal_id };
    let options = mongodb::options::ReplaceOptions::builder().upsert(true).build();
    if let Err(e) = coll.replace_one(filter, &result.data).with_options(options).await {
        log::warn!("Failed to cache random anime: {}", e);
    }

    Ok(serde_json::to_value(result).map_err(|e| ApiError::InternalServer(e.to_string()))?)
}

pub async fn get_anime_by_id_service(
    client: &Client,
    id: i32,
) -> Result<serde_json::Value, ApiError> {
    let coll: Collection<Document> = client.database(DB_NAME).collection(ANIME_COLL_NAME);

    let pipeline = vec![
        doc! { "$match": { "mal_id": id } },
        doc! {
            "$lookup": {
                "from": PRODUCERS_COLL_NAME,
                "localField": "producer_ids",   // ← обновили
                "foreignField": "mal_id",
                "as": "producers"
            }
        },
    ];

    let mut cursor = coll.aggregate(pipeline).await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    match cursor.try_next().await.map_err(|e| ApiError::InternalServer(e.to_string()))? {
        Some(doc) => Ok(serde_json::json!({ "data": doc })),
        None => Err(ApiError::NotFound(format!("Anime {} not found", id))),
    }
}

pub async fn get_anime_by_ids_service(client: &Client, ids_raw: String) -> Result<serde_json::Value, ApiError> {
    let ids: Vec<u32> = ids_raw
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();

    if ids.is_empty() {
        return Err(ApiError::BadRequest("No valid ids provided".to_string()));
    }

    let coll = collection(client);
    let mut cursor = coll
        .find(doc! { "mal_id": { "$in": &ids } })
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let mut data: Vec<AnimeStruct> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        data.push(doc);
    }

    Ok(serde_json::json!({ "data": data }))
}

pub async fn get_anime_by_query_service(
    config: web::Data<AppConfig>,
    client: &Client,
    params: AnimeSearchParams,
) -> Result<AnimeSearchResponse, ApiError> {
    let result = search_anime(&config, params.q).await?;
    let anime_coll = collection(client);
    let producer_coll = producers_service::collection(client);

    let list: Vec<AnimePopulated> = result.data.iter().map(|raw| AnimePopulated {
        anime: AnimeStruct {
            producers: None,
            ..raw.anime.clone()
        },
        producers: raw.producers.clone(),
    }).collect();

    actix_web::rt::spawn(async move {
        cache_anime_list(&anime_coll, &producer_coll, &list).await;
    });

    Ok(result)
}

pub async fn search_anime_in_local_db_service(
    client: &Client,
    params: AnimeSearchParams,
) -> Result<Vec<AnimeStruct>, ApiError> {
    let coll = collection(client);
    let regex = doc! {
        "titles.english": {
            "$regex": &params.q,
            "$options": "i"
        }
    };

    let mut cursor = coll.find(regex).await?;
    let mut results: Vec<AnimeStruct> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        results.push(doc);
    }

    Ok(results)
}

pub async fn get_anime_list_service(
    client: &Client,
    info: AnimeListParams,
) -> Result<AnimeSearchResponse, ApiError> {
    let limit = info.limit.min(100);
    let page = info.page.max(1);
    let skip = (page - 1) * limit;

    let sort_field = match info.sort_by.as_ref().unwrap_or(&AnimeListSortBy::Score) {
        AnimeListSortBy::Score => "score",
        AnimeListSortBy::Rank => "rank",
        AnimeListSortBy::Popularity => "popularity",
        AnimeListSortBy::Year => "year",
        AnimeListSortBy::Episodes => "episodes",
    };

    let sort_dir: i32 = match info.order.as_ref().unwrap_or(&SortOrder::Desc) {
        SortOrder::Asc => 1,
        SortOrder::Desc => -1,
    };

    let coll: Collection<Document> = client.database(DB_NAME).collection(ANIME_COLL_NAME);
    let total = client
        .database(DB_NAME)
        .collection::<AnimeStruct>(ANIME_COLL_NAME)
        .count_documents(doc! {})
        .await?;

    let pipeline = vec![
        doc! { "$sort": { sort_field: sort_dir } },
        doc! { "$skip": skip as i64 },
        doc! { "$limit": limit as i64 },
        doc! {
            "$lookup": {
                "from": PRODUCERS_COLL_NAME,
                "localField": "producer_ids",
                "foreignField": "mal_id",
                "as": "producers"
            }
        },
    ];

    let data: Vec<AnimePopulated> = coll
        .aggregate(pipeline)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .try_collect::<Vec<Document>>()
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .into_iter()
        .map(|doc| {
            mongodb::bson::from_document::<AnimePopulated>(doc)
                .map_err(|e| ApiError::InternalServer(e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let last_visible_page = (total as f64 / limit as f64).ceil() as i32;
    Ok(AnimeSearchResponse {
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
    })
}

pub async fn create_anime_service(
    client: &Client,
    body: CreateAnimeRequest,
) -> Result<serde_json::Value, ApiError> {
    let coll = collection(client);

    if coll
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
        producers: body.producer_ids,
        studios: body.studios,
        genres: body.genres,
    };

    coll.insert_one(&anime)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    Ok(serde_json::json!({ "data": anime }))
}

pub async fn update_anime_service(
    client: &Client,
    id: i32,
    body: UpdateAnimeRequest,
) -> Result<serde_json::Value, ApiError> {
    let coll = collection(client);
    let mut set_doc = doc! {};

    macro_rules! set_if_some {
        ($field:expr, $key:expr) => {
            if let Some(val) = $field {
                let bson_val = mongodb::bson::to_bson(&val)
                    .map_err(|e| ApiError::InternalServer(e.to_string()))?;
                set_doc.insert($key, bson_val);
            }
        };
    }

    set_if_some!(body.url, "url");
    set_if_some!(body.titles, "titles");
    set_if_some!(body.r#type, "type");
    set_if_some!(body.episodes, "episodes");
    set_if_some!(body.status, "status");
    set_if_some!(body.airing, "airing");
    set_if_some!(body.rating, "rating");
    set_if_some!(body.score, "score");
    set_if_some!(body.scored_by, "scored_by");
    set_if_some!(body.rank, "rank");
    set_if_some!(body.popularity, "popularity");
    set_if_some!(body.synopsis, "synopsis");
    set_if_some!(body.background, "background");
    set_if_some!(body.year, "year");
    set_if_some!(body.genres, "genres");
    set_if_some!(body.studios, "studios");
    set_if_some!(body.producers, "producer_ids");

    if set_doc.is_empty() {
        return Err(ApiError::BadRequest("No fields to update".into()));
    }

    let result = coll
        .find_one_and_update(doc! { "mal_id": id }, doc! { "$set": set_doc })
        .return_document(mongodb::options::ReturnDocument::After)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    match result {
        Some(updated) => Ok(serde_json::json!({ "data": updated })),
        None => Err(ApiError::NotFound(format!("Anime with id {} not found", id))),
    }
}

pub async fn get_characters_service(config: web::Data<AppConfig>, id: i32) -> Result<AnimeCharactersResponse, ApiError> {
    get_anime_characters(&config, id).await
}
