#![warn(clippy::all, clippy::nursery, clippy::unwrap_used, clippy::perf)]

use std::path::PathBuf;

use clap::Parser;
use djrip::Library;
use url::Url;

#[derive(Parser, Debug)]
#[command()]
/// DJrip - tool for ripping and organizing your music library
struct Args {
    #[arg(short, long, action = clap::ArgAction::Count)]
    /// Show more detailed output (can be used multiple times)
    verbose: u8,

    #[arg(short, long)]
    /// Supress output
    quiet: bool,

    #[arg(short, long, name = "PATH", value_parser = parse_config_path)]
    /// Override default configuration file path
    config: Option<PathBuf>,

    #[command(subcommand)]
    action: Action,
}

#[derive(Parser, Debug)]
enum Action {
    /// Synchonize library, downloading new/missing tracks
    Sync,
    /// Add a playlist to the library
    Add {
        /// URL of the playlist to add
        #[arg(value_parser = parse_url)]
        url: Url,

        #[arg(short, long)]
        /// Override name of the playlist
        name: Option<String>,

        #[arg(short, long)]
        /// Override subfolder to put the playlist in
        subfolder: Option<PathBuf>,
    },
}

impl Action {
    pub async fn do_action(self, djrip: &mut Library) -> anyhow::Result<()> {
        match self {
            Self::Sync => djrip.sync().await?,
            Self::Add {
                url,
                name,
                subfolder,
            } => djrip.add(url, name, subfolder).await?,
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init()?;

    let args = Args::parse();

    if args.quiet {
        log::set_max_level(log::LevelFilter::Off);
    } else {
        match args.verbose {
            0 => log::set_max_level(log::LevelFilter::Info),
            1 => log::set_max_level(log::LevelFilter::Debug),
            _ => log::set_max_level(log::LevelFilter::Trace),
        }
    }
    log::trace!("provided CLI arguments: {:?}", args);

    let mut djrip = if let Some(config_path) = &args.config {
        Library::new(config_path.clone())?
    } else {
        let default_config_path = {
            #[cfg(debug_assertions)]
            {
                PathBuf::from("config.yml")
            }
            #[cfg(not(debug_assertions))]
            {
                PathBuf::from(std::env::var("HOME")?).join(".djrip.yml")
            }
        };
        Library::new(default_config_path)?
    };

    args.action.do_action(&mut djrip).await?;

    Ok(())
}

fn parse_config_path(arg: &str) -> anyhow::Result<PathBuf> {
    let config_path = std::path::PathBuf::from(arg);
    if config_path.exists() && config_path.is_dir() {
        return Err(anyhow::anyhow!(
            "path is a directory, it shoud be a .yml file, or non-existent",
        ));
    }
    Ok(config_path)
}

fn parse_url(arg: &str) -> anyhow::Result<Url> {
    let url = url::Url::parse(arg)?;
    Ok(url)
}
