use url::Url;

use super::{config::YoutubeConfig, Client};

pub struct YoutubeClient {
    config: YoutubeConfig,
}

impl YoutubeClient {
    pub fn new(config: YoutubeConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl Client for YoutubeClient {
    async fn get_playlist_from_url(&self, query: &Url) -> anyhow::Result<super::Playlist> {
        todo!()
    }
    async fn save_config(&self, configs: super::ClientConfigs) -> anyhow::Result<()> {
        configs.write().await.youtube = Some(self.config.clone());
        Ok(())
    }
}
