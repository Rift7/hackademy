use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub server_addr: String,
    pub session_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or("sqlite://hackademy.db".to_string()),
            server_addr: env::var("HACKADEMY_ADDR").unwrap_or("0.0.0.0:3000".to_string()),
            session_secret: env::var("SESSION_SECRET").unwrap_or("random_secret".to_string()),
        }
    }
}