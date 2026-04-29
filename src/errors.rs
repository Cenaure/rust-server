use actix_web::{
    error, http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::Display;
use serde::Serialize;

//region: ---API Errors
#[derive(Debug, Display)]
pub enum ApiError {
    #[display("internal server error: {_0}")]
    InternalServer(String),

    #[display("not found: {_0}")]
    NotFound(String),

    #[display("bad request: {_0}")]
    BadRequest(String),

    #[display("unauthorized: {_0}")]
    Unauthorized(String),

    #[display("forbidden: {_0}")]
    Forbidden(String),

    #[display("token expired")]
    TokenExpired,

    #[display("validation error: {_0}")]
    ValidationError(String),

    #[display("bad gateway: {_0}")]
    BadGateway(String),

    #[display("too many requests: {_0}")]
    TooManyRequests(String),

    #[display("conflict: {_0}")]
    Conflict(String),
}
//endregion: ---API Errors

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::TokenExpired => StatusCode::UNAUTHORIZED,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::BadGateway(_) => StatusCode::BAD_GATEWAY,
            ApiError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .insert_header(("Access-Control-Allow-Origin", "http://localhost:4200"))
            .insert_header(("Access-Control-Allow-Credentials", "true"))
            .json(ErrorBody { error: self.to_string() })
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(err: mongodb::error::Error) -> Self {
        ApiError::InternalServer(err.to_string())
    }
}