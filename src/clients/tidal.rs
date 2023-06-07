use std::sync::Arc;

use tokio::sync::RwLock;
use url::Url;

use super::{config::TidalConfig, Client};

pub struct TidalClient {
    config: TidalConfig,
}

impl TidalClient {
    pub fn new(config: TidalConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl Client for TidalClient {
    async fn get_playlist_from_url(&self, query: &Url) -> anyhow::Result<super::Playlist> {
        todo!()
    }
    async fn save_config(&self, configs: super::ClientConfigs) -> anyhow::Result<()> {
        configs.write().await.tidal = Some(self.config.clone());
        Ok(())
    }
}
