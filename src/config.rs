use std::{
    env,
    path::{Path, PathBuf},
};

use figment::{
    providers::{Format, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};

/// Name of the environment variable to lookup for config path
pub const CONFIG_VAR: &str = "MEDIAWHALER_CONFIG";
/// The name of the config file to use
const CONFIG_FILENAME: &str = "config.yaml";

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("config not found: {0}")]
    ConfigNotFoundError(String),
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

impl ConfigError {
    fn config_file_does_not_exists(path: &Path) -> Self {
        Self::ConfigNotFoundError(format!("file {} does not exists", path.display()))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub logs: ConfigLog,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub http: ConfigHttp,
    pub https: Option<ConfigHttps>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigLog {
    pub location: Option<PathBuf>,
    pub term: Option<ConfigTerm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigTerm {
    StdOut,
    StdErr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHttp {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHttps {
    pub port: u16,
    pub cert: PathBuf,
    pub key: PathBuf,
}

impl Default for ConfigHttp {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

impl Config {
    fn find_config_in_path(path: &Path) -> Result<PathBuf, ConfigError> {
        let mut path = path.to_path_buf();
        if path.is_file() && path.ends_with(CONFIG_FILENAME) {
            Ok(path)
        } else if path.exists() {
            path.push(CONFIG_FILENAME);
            match path.is_file() {
                true => Ok(path),
                false => Err(ConfigError::config_file_does_not_exists(&path)),
            }
        } else {
            Err(ConfigError::config_file_does_not_exists(&path))
        }
    }

    fn path() -> Result<PathBuf, ConfigError> {
        if let Ok(path) = env::var(CONFIG_VAR) {
            let path = PathBuf::from(path);
            Self::find_config_in_path(&path)
        } else {
            Err(ConfigError::ConfigNotFoundError(format!(
                "{CONFIG_VAR} is not defined and config file not found"
            )))
        }
    }

    fn figment() -> Result<Figment, ConfigError> {
        let config_path = Self::path()?;
        Ok(Figment::from(Serialized::defaults(Config::default()))
            .merge(figment::providers::Yaml::file(config_path))
            .merge(figment::providers::Env::prefixed("MEDIAWHALER_")))
    }

    pub fn new() -> Result<Config, ConfigError> {
        Self::figment()?
            .extract()
            .map_err(|e| ConfigError::ParsingError(format!("{e}")))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn config_http_port() {
        figment::Jail::expect_with(|jail| {
            jail.set_env(CONFIG_VAR, ".");
            jail.create_file(
                "config.yaml",
                r#"
                network:
                    http:
                        port: 3000
            "#,
            )?;

            let config = Config::new().map_err(|e| format!("{e}"))?;
            assert_eq!(config.network.http.port, 3000);
            Ok(())
        })
    }
}
