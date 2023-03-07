use std::{
    env,
    path::{Path, PathBuf},
};

use crate::directories::Dirs;

use figment::{
    providers::{Data, Format, Serialized, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

/// Name of the environment variable to lookup for config path
pub const CONFIG_VAR: &str = "MEDIAWHALER_CONFIG";
/// The name of the config file to use
const CONFIG_FILENAME: &str = "config.yaml";

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub logs: ConfigLog,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub http: ConfigHttp,
    pub https: Option<ConfigHttps>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConfigLog {
    pub location: Option<PathBuf>,
    pub term: Option<ConfigTerm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn find_config_in_path(path: &Path) -> Option<PathBuf> {
        let mut path = path.to_path_buf();
        if path.is_file() && path.ends_with(CONFIG_FILENAME) {
            Some(path)
        } else if path.exists() {
            path.push(CONFIG_FILENAME);
            path.is_file().then_some(path)
        } else {
            None
        }
    }

    fn from_env() -> Option<Data<Yaml>> {
        env::var(CONFIG_VAR).ok().as_ref().and_then(|path| {
            Self::find_config_in_path(Path::new(path)).map(figment::providers::Yaml::file)
        })
    }

    fn from_sys_config() -> Option<Data<Yaml>> {
        Dirs::sys_config_path().and_then(|path| {
            let mut path = path.to_path_buf();
            path.push(CONFIG_FILENAME);
            path.exists()
                .then_some(figment::providers::Yaml::file(path))
        })
    }

    fn from_user_config(dirs: &Dirs) -> Option<Data<Yaml>> {
        Self::find_config_in_path(dirs.config()).map(figment::providers::Yaml::file)
    }

    fn from_current_dir() -> Option<Data<Yaml>> {
        std::env::current_dir().as_mut().ok().and_then(|dir| {
            dir.push(CONFIG_FILENAME);
            dir.exists().then_some(figment::providers::Yaml::file(dir))
        })
    }

    fn figment(dirs: &Dirs) -> Figment {
        let mut figment = Figment::from(Serialized::defaults(Config::default()));

        if let Some(provider) = Self::from_sys_config() {
            figment = figment.merge(provider);
        }

        if let Some(provider) = Self::from_user_config(dirs) {
            figment = figment.merge(provider);
        }

        if let Some(provider) = Self::from_env() {
            figment = figment.merge(provider);
        }

        if let Some(provider) = Self::from_current_dir() {
            figment = figment.merge(provider);
        }

        figment = figment.merge(figment::providers::Env::prefixed("MEDIAWHALER_"));

        dbg!(figment)
    }

    pub fn new(dirs: &Dirs) -> Result<Config, ConfigError> {
        Self::figment(dirs)
            .extract()
            .map_err(|e| ConfigError::ParsingError(e.to_string()))
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

            let dirs = Dirs::new().ok_or("unable to create dirs".to_string())?;
            let config = Config::new(&dirs).map_err(|e| format!("{e}"))?;
            assert_eq!(config.network.http.port, 3000);
            Ok(())
        })
    }
}
