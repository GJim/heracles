use clap::{CommandFactory, Parser, Subcommand};
use snafu::ResultExt;
use std::{io::Write, path::PathBuf};
use tokio::runtime::Runtime;

use crate::{
    config::{self, Config},
    error::{self, Error},
    shadow,
};

#[derive(Parser)]
#[command(
    name = heracles_base::PROJECT_NAME,
    author,
    version,
    long_version = shadow::CLAP_LONG_VERSION,
    about,
    long_about = None
)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Option<Commands>,

    #[clap(
        long = "config",
        short = 'c',
        env = "CEXFLOW_CONFIG_FILE_PATH",
        help = "Specify a configuration file"
    )]
    config_file_path: Option<PathBuf>,
}

impl Default for Cli {
    #[inline]
    fn default() -> Self {
        Self::parse()
    }
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    #[clap(about = "Print version information")]
    Version,

    #[clap(about = "Output shell completion code for the specified shell (bash, zsh, fish)")]
    Completions { shell: clap_complete::Shell },

    #[command(about = "Output default configuration")]
    DefaultConfig,

    #[command(about = "Subscribe to CreatePoolEvent from pump_amm program")]
    Subscribe,
}

impl Cli {
    #[allow(clippy::unnecessary_wraps)]
    pub fn run(self) -> Result<(), Error> {
        match self.commands {
            Some(Commands::Version) => {
                std::io::stdout()
                    .write_all(Self::command().render_long_version().as_bytes())
                    .expect("Failed to write to stdout");
            }
            Some(Commands::Completions { shell }) => {
                let mut app = Self::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
            }
            Some(Commands::DefaultConfig) => {
                let config_text = serde_yaml::to_string(&Config::default())
                    .expect("Failed to serialize config as yaml");
                std::io::stdout()
                    .write_all(config_text.as_bytes())
                    .expect("Failed to write to stdout");
            }
            Some(Commands::Subscribe) => {
                let config = self.load_config()?;
                config.log.registry();
                let _unused = Runtime::new()
                    .context(error::InitializeTokioRuntimeSnafu)?
                    .block_on(async move { listener::subscribe().await });
            }
            _ => {
                Self::command().print_help().expect("Failed to write to stdout");
            }
        };
        Ok(())
    }

    #[allow(dead_code)]
    fn load_config(&self) -> Result<Config, config::Error> {
        let config_file_path =
            &self.config_file_path.clone().unwrap_or_else(Config::search_config_file_path);
        Config::load(config_file_path)
    }
}
