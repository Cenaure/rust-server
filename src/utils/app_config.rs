
#[derive(Clone)]
pub struct AppConfig {
    pub jwt_secret: Vec<u8>,
    pub database_url: String,
}