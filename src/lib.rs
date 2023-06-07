use std::path::PathBuf;
use url::Url;

mod clients;
mod config;

use config::{Config, Playlist};

/// DJrip library manager
pub struct Library {
    config: Config,
    clients: clients::Clients,
}

impl Library {
    pub fn new(config_path: PathBuf) -> anyhow::Result<Self> {
        let config = Config::load(config_path)?;
        Ok(Self {
            clients: clients::Clients::new(config.clients.clone()),
            config,
        })
    }

    pub async fn sync(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn add(
        &mut self,
        url: Url,
        name: Option<String>,
        subfolder: Option<PathBuf>,
    ) -> anyhow::Result<()> {
        let playlist = self.clients.get_playlist_from_url(&url).await?;

        let entry = self.config.playlists.iter_mut().find(|p| p.url == url);

        if let Some(entry) = entry {
            log::info!("playlist already exists: {:?}", entry);

            if let Some(name) = name.clone() {
                entry.name = name;
            }

            entry.subfolder = subfolder.clone();
        } else {
            let entry = Playlist {
                name: name.clone().unwrap_or(playlist.name.clone()),
                url: playlist.url.clone(),
                subfolder,
            };

            playlist
                .download(
                    &entry
                        .subfolder
                        .clone()
                        .unwrap_or(PathBuf::from(entry.name.clone())),
                )
                .await?;

            self.config.playlists.push(entry);
        }
        Ok(())
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        self.config
            .save()
            .expect("failed to save config on library destruction");
    }
}
