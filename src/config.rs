use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub backend_url: Option<String>,
    pub api_key: Option<String>,
    pub enabled_patterns: Vec<String>,
    pub entropy_threshold: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backend_url: None,
            api_key: None,
            enabled_patterns: vec![
                "aws_access_key".to_string(),
                "github_token".to_string(),
                "api_key".to_string(),
                "jwt_token".to_string(),
            ],
            entropy_threshold: 0.7,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_paths = [
            "./aegis.toml",
            "~/.config/aegis/config.toml",
            "/etc/aegis/config.toml",
        ];

        for path in config_paths.iter() {
            if Path::new(path).exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }

        Config::default()
    }
}