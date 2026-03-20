
#[derive(Clone)]
pub struct AppConfig {
    pub jwt_secret: Vec<u8>,
    pub database_url: String,
    pub jikan_api_url: String,
    pub http_client: reqwest::Client,
}