use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use url::Url;

pub(crate) mod config;
pub mod tidal;
pub mod youtube;

pub type ClientConfigs = Arc<RwLock<self::config::Clients>>;

pub struct Clients {
    configs: ClientConfigs,
    youtube: Option<youtube::YoutubeClient>,
    tidal: Option<tidal::TidalClient>,
}

impl Clients {
    pub fn new(configs: ClientConfigs) -> Self {
        Self {
            configs,
            youtube: None,
            tidal: None,
        }
    }

    pub async fn get_playlist_from_url(&mut self, query: &Url) -> anyhow::Result<Playlist> {
        match query.host_str() {
            Some("www.tidal.com" | "listen.tidal.com" | "tidal.com") => {
                if let Some(tidal) = &self.tidal {
                    tidal.get_playlist_from_url(query).await
                } else {
                    let tidal_config = {
                        let configs = self.configs.read().await;
                        configs.tidal.clone()
                    }
                    .unwrap_or_default();

                    let tidal = tidal::TidalClient::new(tidal_config);
                    self.tidal = Some(tidal);
                    self.tidal
                        .as_ref()
                        .unwrap()
                        .get_playlist_from_url(query)
                        .await
                }
            }
            Some("youtube.com" | "www.youtube.com") => {
                if let Some(youtube) = &self.youtube {
                    youtube.get_playlist_from_url(query).await
                } else {
                    let yt_config = {
                        let configs = self.configs.read().await;
                        configs.youtube.clone()
                    }
                    .unwrap_or_default();

                    let youtube = youtube::YoutubeClient::new(yt_config);
                    self.youtube = Some(youtube);
                    self.youtube
                        .as_ref()
                        .unwrap()
                        .get_playlist_from_url(query)
                        .await
                }
            }
            _ => anyhow::bail!("unsupported service: {}", query),
        }
    }
}

#[async_trait::async_trait]
trait Client {
    async fn get_playlist_from_url(&self, query: &Url) -> anyhow::Result<Playlist>;
    async fn save_config(&self, configs: ClientConfigs) -> anyhow::Result<()>;
}

pub struct Playlist {
    client: Box<dyn Client>,
    pub name: String,
    pub url: Url,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub async fn download(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let futures = self.tracks.iter().map(|track| {
            let track_path = path.join(format!("{} - {}", track.artist, track.title));
            track.download(track_path)
        });
        futures_util::future::try_join_all(futures).await?;
        Ok(())
    }
}

pub struct Track {
    pub artist: String,
    pub title: String,
    pub url: Url,
}
impl Track {
    pub async fn download(&self, path: PathBuf) -> anyhow::Result<()> {
        anyhow::bail!("not implemented")
    }
}
