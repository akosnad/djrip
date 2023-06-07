use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Clients {
    pub youtube: Option<YoutubeConfig>,
    pub tidal: Option<TidalConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct YoutubeConfig {}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TidalConfig {}
