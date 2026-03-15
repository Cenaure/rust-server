use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub username: String,
    pub email: String,
}

pub fn encode_jwt(
    username: &str,
    email: &str,
    id: ObjectId,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = Duration::minutes(15);

    let claims = Claims {
        sub: id.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + exp).timestamp() as usize,
        username: username.to_owned(),
        email: email.to_owned(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn decode_jwt(
    token: &str,
    secret: &[u8],
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    decode(token, &DecodingKey::from_secret(secret), &validation)
}