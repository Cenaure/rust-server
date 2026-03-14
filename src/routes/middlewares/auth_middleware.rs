use crate::errors::ApiError;
use crate::utils::app_config::AppConfig;
use crate::utils::jwt::decode_jwt;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::web;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let config = req
        .app_data::<web::Data<AppConfig>>()
        .ok_or_else(|| ApiError::InternalServer("Config not found".to_string()))?;

    let token = req
        .headers()
        .get("AUTHORIZATION")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| ApiError::Unauthorized("Invalid or missing Authorization header".to_string()))?;

    decode_jwt(token, &config.jwt_secret)
        .map_err(|_| ApiError::Unauthorized("Invalid token".to_string()))?;

    next.call(req).await
}