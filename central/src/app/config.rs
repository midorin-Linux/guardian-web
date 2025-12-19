use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

fn default_bind_port() -> u16 {
    3000
}
fn default_database_url() -> String {
    "sqlite:./guardian.db".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_bind_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_bind_port(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(rename = "log_level", default = "default_log_level")]
    pub log_level: String,

    #[serde(rename = "database_url", default = "default_database_url")]
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = ConfigBuilder::builder()
            .add_source(
                File::with_name(".env")
                    .format(config::FileFormat::Ini)
                    .required(false),
            )
            .add_source(
                File::with_name("central.toml")
                    .required(false)
                    .format(config::FileFormat::Toml),
            )
            .add_source(Environment::default().separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
