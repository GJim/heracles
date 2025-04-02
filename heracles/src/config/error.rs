use std::path::PathBuf;

use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Could not open config from {filename:?}, error: {source}"))]
    OpenConfig { filename: PathBuf, source: std::io::Error },

    #[snafu(display("Could not parse config from {filename:?}, error: {source}"))]
    ParseConfig { filename: PathBuf, source: serde_yaml::Error },

    #[snafu(display("Could not resolve file path {file_path:?}, error: {source}"))]
    ResolveFilePath { file_path: PathBuf, source: std::io::Error },
}
