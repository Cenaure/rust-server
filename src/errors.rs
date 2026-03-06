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
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorBody { error: self.to_string() })
    }
}
