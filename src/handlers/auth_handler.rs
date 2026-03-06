use crate::errors::ApiError;
use crate::handlers::users_handler::USERS_COLL_NAME;
use crate::handlers::DB_NAME;
use crate::models::{User, UserDTO, UserSignIn, UserSignUp};
use actix_web::{get, post, web, HttpResponse};
use mongodb::bson::doc;
use mongodb::{Client, Collection};

#[post("/sign-in")]
pub async fn sign_in(client: web::Data<Client>, sign_in_dto: web::Json<UserSignIn>) -> Result<HttpResponse, ApiError> {
    let collection: Collection<User> = client.database(DB_NAME).collection(USERS_COLL_NAME);

    let user = collection
        .find_one(doc! { "$or":
            [
                {"email": &sign_in_dto.username_or_email},
                {"username": &sign_in_dto.username_or_email}
            ]
        })
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    if user.is_none() {
        return Err(ApiError::BadRequest("Wrong email or password".to_string()))
    }

    match bcrypt::verify(&sign_in_dto.password, &user.unwrap().password) {
        Ok(true) => {}
        Ok(false) => return Err(ApiError::BadRequest("Wrong email or password".to_string())),
        Err(_) => return Err(ApiError::InternalServer("bcrypt error".to_string())),
    }

    todo!()
}

#[post("/sign-up")]
pub async fn sign_up(client: web::Data<Client>, sign_up_dto: web::Json<UserSignUp>) -> Result<HttpResponse, ApiError> {
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
        id: 0,  // MongoDB сам назначит _id, это поле можно убрать
        username: sign_up_dto.username.clone(),
        email: sign_up_dto.email.clone(),
        password: bcrypt::hash(&sign_up_dto.password, 12)
            .map_err(|e| ApiError::InternalServer(e.to_string()))?,
        groups: vec![],
        last_login: None,
    };

    collection
        .insert_one(&new_user)
        .await
        .map_err(|e| ApiError::InternalServer(e.to_string()))?;

    Ok(HttpResponse::Created().json(UserDTO {
        id: None,
        username: new_user.username,
        email: new_user.email,
        groups: new_user.groups,
        last_login: new_user.last_login,
    }))
}

#[get("/logout")]
pub async fn logout(client: web::Data<Client>) -> Result<HttpResponse, ApiError> {
    todo!()
}