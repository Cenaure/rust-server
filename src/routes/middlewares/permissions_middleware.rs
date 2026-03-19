use crate::errors::ApiError;
use crate::handlers::users_handler::USERS_COLL_NAME;
use crate::handlers::DB_NAME;
use crate::models::User;
use crate::utils::app_config::AppConfig;
use crate::utils::jwt::decode_jwt;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::web;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};

pub async fn require_permissions(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    required: &[&str],
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let config = req
        .app_data::<web::Data<AppConfig>>()
        .ok_or_else(|| ApiError::InternalServer("Config not found".to_string()))?;

    let client = req
        .app_data::<web::Data<Client>>()
        .ok_or_else(|| ApiError::InternalServer("DB client not found".to_string()))?;

    let token = req
        .cookie("access_token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get("AUTHORIZATION")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .map(|v| v.to_string())
        })
        .ok_or_else(|| ApiError::Unauthorized("Missing token".to_string()))?;

    let claims = decode_jwt(&token, &config.jwt_secret)
        .map_err(|_| ApiError::Unauthorized("Invalid token".to_string()))?;

    let user_id = ObjectId::parse_str(&claims.claims.sub)
        .map_err(|_| ApiError::Unauthorized("Invalid user id in token".to_string()))?;

    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let user = collection
        .find_one(doc! {"_id": user_id})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .ok_or_else(|| ApiError::Unauthorized("User not found".to_string()))?;

    let user_permissions: Vec<&str> = user
        .groups
        .iter()
        .flat_map(|g| g.permissions.iter().map(|p| p.as_str()))
        .collect();

    let has_permissions = required
        .iter()
        .all(|req| user_permissions.contains(req));

    if !has_permissions {
        return Err(ApiError::Forbidden("Insufficient permissions".to_string()).into());
    }

    next.call(req).await
}