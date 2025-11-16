use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

fn default_bind_port() -> u16 {
    3000
}

fn default_log_level() -> String {
    "info".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "bind_port", default="default_bind_port")]
    pub port: u16,

    #[serde(rename = "log_level", default="default_log_level")]
    pub log_level: String,
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
                File::with_name("settings.toml")
                    .required(true)
                    .format(config::FileFormat::Toml),
            )
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}