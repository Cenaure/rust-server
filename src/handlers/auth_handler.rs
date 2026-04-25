use crate::errors::ApiError;
use crate::handlers::users_handler::USERS_COLL_NAME;
use crate::handlers::DB_NAME;
use crate::models::{User, UserDTO, UserSignIn, UserSignUp};
use crate::utils::app_config::AppConfig;
use crate::utils::jwt::encode_jwt;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;
use mongodb::bson::{doc, DateTime};
use mongodb::{Client, Collection};

#[utoipa::path(
    post,
    path = "/api/auth/sign-in",
    tag = "Auth",
    request_body = UserSignIn,
    responses(
        (status = 200, description = "Signed in successfully"),
        (status = 400, description = "Wrong email or password"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/sign-in")]
pub async fn sign_in(client: web::Data<Client>, config: web::Data<AppConfig>, sign_in_dto: web::Json<UserSignIn>) -> Result<HttpResponse, ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let user = collection
        .find_one(doc! { "$or":
            [
                {"email": &sign_in_dto.username_or_email},
                {"username": &sign_in_dto.username_or_email}
            ]
        })
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?
        .ok_or(ApiError::BadRequest("Wrong Email or Password".to_string()))?;

    match bcrypt::verify(&sign_in_dto.password, &user.password) {
        Ok(true) => {}
        Ok(false) => return Err(ApiError::BadRequest("Wrong email or password".to_string())),
        Err(_) => return Err(ApiError::InternalServer("bcrypt error".to_string())),
    }

    let user_id = user.id.ok_or(ApiError::BadRequest("User does not exist".to_string()))?;

    let access_token = encode_jwt(&user.username, &user.email, user_id, &config.jwt_secret)
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;;

    collection
        .find_one_and_update(doc! {"_id": user_id}, doc! {"$set": {"last_login": DateTime::from_millis(Utc::now().timestamp_millis())}})
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("access_token", access_token)
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::minutes(30))
                .finish(),
        ).finish()
    )
}
#[utoipa::path(
    post,
    path = "/api/auth/sign-up",
    tag = "Auth",
    request_body = UserSignUp,
    responses(
        (status = 201, description = "User created", body = UserDTO),
        (status = 400, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/sign-up")]
pub async fn sign_up(client: web::Data<Client>, config: web::Data<AppConfig>, sign_up_dto: web::Json<UserSignUp>) -> Result<HttpResponse, ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let existing_user: Option<User> = collection
        .find_one(doc! { "$or":
            [
                {"email": &sign_up_dto.email},
                {"username": &sign_up_dto.username}
            ]
        })
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if existing_user.is_some() {
        return Err(ApiError::BadRequest("User already exists".to_string()));
    }

    let new_user = User {
        id: None,
        username: sign_up_dto.username.clone(),
        email: sign_up_dto.email.clone(),
        password: bcrypt::hash(&sign_up_dto.password, 12)
            .map_err(|e| ApiError::InternalServer(e.to_string()))?,
        groups: vec![],
        last_login: None,
    };

    let insert_result = collection
        .insert_one(&new_user)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    let inserted_id = insert_result
        .inserted_id
        .as_object_id()
        .ok_or(ApiError::InternalServer("Failed to get inserted id".to_string()))?;

    let access_token = encode_jwt(&new_user.username, &new_user.email, inserted_id, &config.jwt_secret)
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;;

    Ok(HttpResponse::Created()
        .cookie(
            Cookie::build("access_token", access_token)
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::minutes(30))
                .finish(),
        )
        .json(UserDTO {
            id: None,
            username: new_user.username,
            email: new_user.email,
            groups: new_user.groups,
            last_login: new_user.last_login,
        })
    )
}

#[utoipa::path(
    get,
    path = "/api/auth/logout",
    tag = "Auth",
    responses(
        (status = 200, description = "Logged out")
    )
)]
#[get("/logout")]
pub async fn logout() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok()
        .cookie(Cookie::build("access_token", "")
            .path("/")
            .max_age(time::Duration::ZERO)
            .finish()
        )
        .finish()
    )
}