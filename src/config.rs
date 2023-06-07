use std::{fs, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::clients::config::*;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing, skip_deserializing)]
    /// Path to the config file
    path: PathBuf,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    /// List of playlists to manage
    pub playlists: Vec<Playlist>,

    #[serde(
        serialize_with = "serialize_clients",
        deserialize_with = "deserialize_clients"
    )]
    /// Configs for the download clients
    pub clients: Arc<RwLock<Clients>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    /// The name of the playlist folder
    pub name: String,

    #[serde(deserialize_with = "deserialize_url", serialize_with = "serialize_url")]
    /// Source of the playlist
    pub url: Url,

    #[serde(deserialize_with = "deserialize_option_path", default)]
    /// If set, the playlist folder will be saved inside this directory
    pub subfolder: Option<PathBuf>,
}

impl Config {
    pub(crate) fn load(path: PathBuf) -> anyhow::Result<Config> {
        if !path.exists() {
            log::trace!("config file does not exist, loading default config");
            return Ok(Self {
                path,
                ..Config::default()
            });
        }
        let file =
            fs::File::open(path.clone()).map_err(|e| anyhow!("failed to open config file: {e}"))?;
        let config = Self {
            path,
            ..serde_yaml::from_reader(file)
                .map_err(|e| anyhow::anyhow!("failed to read config file: {e}"))?
        };
        log::trace!("loaded config from {}: {:?}", config.path.display(), config);
        Ok(config)
    }

    pub(crate) fn save(&self) -> anyhow::Result<()> {
        if !self.path.exists() {
            if let Some(parent) = self.path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)
                        .map_err(|e| anyhow!("failed to create config parent dirs: {e}"))?;
                }
            }
        }

        let file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path.clone())
            .map_err(|e| anyhow!("failed to create config file: {e}"))?;

        log::trace!("saving config to {}", self.path.display());
        serde_yaml::to_writer(file, self)
            .map_err(|e| anyhow!("failed to write config to file: {e}"))?;
        Ok(())
    }
}

fn deserialize_option_path<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(Some(PathBuf::from(s)));
    }
    Ok(None)
}

fn serialize_url<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(url.as_str())
}

fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
}

fn serialize_clients<S>(clients: &Arc<RwLock<Clients>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    clients
        .try_read()
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}

fn deserialize_clients<'de, D>(deserializer: D) -> Result<Arc<RwLock<Clients>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let clients: Clients = Deserialize::deserialize(deserializer)?;
    Ok(Arc::new(RwLock::new(clients)))
}
