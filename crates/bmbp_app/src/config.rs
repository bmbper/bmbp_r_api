use serde::Deserialize;
use std::fs;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub app: BmbpConfig,
    pub server: ServerConfig,
    pub datasource: DataSourceConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct BmbpConfig {
    pub code: String,
    pub name: String,
    pub login_name: String,
    pub nav_name: String,
    pub copy_right: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub env: String,
    pub log_level: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct DataSourceConfig {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub init_size: u32,
}

pub fn load_app_config(config_path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: AppConfig = toml::from_str(&config_content)?;
    Ok(config)
}
