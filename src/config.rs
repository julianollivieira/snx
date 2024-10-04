use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub app: AppConfig,
}
