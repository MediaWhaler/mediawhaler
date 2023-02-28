use std::{
    env,
    path::{Path, PathBuf},
};

use clap::Parser;
use figment::{
    providers::{Serialized, Yaml, Format, Json},
    value::{Dict, Map},
    Error, Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    http: ConfigHttp,
    https: Option<ConfigHttps>,
}

#[derive(Serialize, Deserialize)]
struct ConfigHttp {
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct ConfigHttps {
    port: u16,
    cert: PathBuf,
    key: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http: Default::default(),
            https: Default::default(),
        }
    }
}

impl Default for ConfigHttp {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

// // Make `Config` a provider itself for composability.
// impl Provider for Config {
//     fn metadata(&self) -> Metadata {
//         Metadata::named("Media Whaler")
//     }

//     fn data(&self) -> Result<Map<Profile, Dict>, Error>  {
//         figment::providers::Serialized::defaults(Config::default()).data()
//     }

//     fn profile(&self) -> Option<Profile> {
//         // Optionally, a profile that's selected by default.
//         Some(Profile::Default)
//     }
// }

// impl Config {
//     // Allow the configuration to be extracted from any `Provider`.
//     fn from<T: Provider>(provider: T) -> Result<Config, Error> {
//         Figment::from(provider).extract()
//     }

//     // Provide a default provider, a `Figment`.
//     fn figment() -> Figment {
//         use figment::providers::Env;

//         // In reality, whatever the library desires.
//         Figment::from(Config::default()).merge(Env::prefixed("APP_"))
//     }
// }

enum SupportedConfig {
    JSON(PathBuf),
    YAML(PathBuf),
}

impl Config {
    fn path() -> Result<PathBuf, anyhow::Error> {
        let yaml_config = "config.yaml";
        let json_config = "config.json";
        if let Ok(path) = env::var("MEDIAWHALER_CONFIG") {
            let mut path = PathBuf::from(path);
            path.push(&yaml_config);
            if path.exists() {
                return Ok(path);
            }
        }
        // else if let Ok(path) = directories::ProjectDirs::from("com", "mediawhaler", "MediaWhaler")
        //     .ok_or("Failed to create project dir")
        //     .and_then(|p| Ok(p.config_dir()))
        // match {
        //     Ok(path) if PathBuf::from(path).exists() => return Ok(PathBuf::from(path)),
        //     _ => (),
        // }
        // let config_path = if let Ok(path) = env::var("MEDIAWHALER_CONFIG") {
        //     PathBuf::from(path)
        // } else if let Ok(path) = directories::ProjectDirs::from("com", "mediawhaler", "MediaWhaler")
        //     .ok_or("Failed to create project dir")
        //     .and_then(|p| Ok(p.config_dir()))
        // {
        //     PathBuf::from(path)
        // } else {
        //     todo!()
        // };
        unimplemented!()
    }
    pub fn figment() -> Result<Figment, anyhow::Error> {
        let config_path = Self::path()?;
        // let merge_from = match config_path.extension() {
        //     Some("yaml") => Yaml::file(config_path),
        //     Some("json") => Json::file(config_path),
        //     _ => return Err("config should be either a json or yaml"),
        // };
        // Figment::from(Serialized::defaults(Config::default())).merge(merge_from)
        Ok(Figment::from(Serialized::defaults(Config::default())))
    }
}
