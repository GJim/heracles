use snafu::Snafu;

use crate::config;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}"))]
    Config { source: config::Error },

    #[snafu(display("Failed to initialize tokio runtime: {source}"))]
    InitializeTokioRuntime { source: tokio::io::Error },

    #[snafu(display("Failed to shutdown tokio runtime: {source}"))]
    ShutdownTokioRuntime { source: tokio_graceful_shutdown::errors::GracefulShutdownError },
}

impl From<config::Error> for Error {
    fn from(source: config::Error) -> Self {
        Self::Config { source }
    }
}

impl From<tokio_graceful_shutdown::errors::GracefulShutdownError> for Error {
    fn from(source: tokio_graceful_shutdown::errors::GracefulShutdownError) -> Self {
        Self::ShutdownTokioRuntime { source }
    }
}

pub trait CommandError {
    fn exit_code(&self) -> exitcode::ExitCode;
}

impl CommandError for Error {
    fn exit_code(&self) -> exitcode::ExitCode {
        match self {
            Self::Config { .. } => exitcode::CONFIG,
            Self::InitializeTokioRuntime { .. } | Self::ShutdownTokioRuntime { .. } => {
                exitcode::IOERR
            }
        }
    }
}
