use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub username: String,
    pub email: String,
    pub id: String,
}

pub fn encode_jwt(
    username: &str,
    email: &str,
    id: ObjectId,
    secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = OffsetDateTime::now_utc();
    let exp = now + time::Duration::minutes(30);

    let claims = Claims {
        iat: now.unix_timestamp() as usize,
        exp: exp.unix_timestamp() as usize,
        username: username.to_owned(),
        email: email.to_owned(),
        id: id.to_string(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn decode_jwt(
    token: &str,
    secret: &[u8],
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode(token, &DecodingKey::from_secret(secret), &Validation::default())
}