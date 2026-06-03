use std::path::PathBuf;

use serde::Deserialize;

use crate::diagnose;

const DEFAULT_SERVER_PORT: u16 = 8765;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_enabled")]
    pub enabled: bool,
    #[serde(default = "default_server_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            enabled: default_server_enabled(),
            port: default_server_port(),
        }
    }
}

fn default_server_enabled() -> bool {
    true
}

fn default_server_port() -> u16 {
    DEFAULT_SERVER_PORT
}

fn config_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(appdata)
        .join("ClaudeCodeUsageMonitor")
        .join("config.toml")
}

pub fn load() -> AppConfig {
    let path = config_path();
    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return AppConfig::default(),
    };

    match toml::from_str::<AppConfig>(&content) {
        Ok(config) => config,
        Err(error) => {
            diagnose::log_error(
                &format!("unable to parse config.toml at {}", path.display()),
                error,
            );
            AppConfig::default()
        }
    }
}
