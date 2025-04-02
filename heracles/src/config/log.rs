use std::{fs::OpenOptions, path::PathBuf};

use serde::{Deserialize, Serialize};
use tracing_subscriber::{
    layer::SubscriberExt, registry::LookupSpan, util::SubscriberInitExt, Layer,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogConfig {
    #[serde(default = "LogConfig::default_file_path")]
    pub file_path: Option<PathBuf>,

    #[serde(default = "LogConfig::default_emit_stdout")]
    pub emit_stdout: bool,

    #[serde(default = "LogConfig::default_emit_stderr")]
    pub emit_stderr: bool,

    #[serde(default = "LogConfig::default_log_filters")]
    pub log_filters: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            file_path: Self::default_file_path(),
            emit_stdout: Self::default_emit_stdout(),
            emit_stderr: Self::default_emit_stderr(),
            log_filters: Self::default_log_filters(),
        }
    }
}

impl LogConfig {
    #[inline]
    #[must_use]
    pub fn default_log_filters() -> String {
        "info".to_string()
    }

    #[inline]
    #[must_use]
    pub const fn default_file_path() -> Option<PathBuf> {
        None
    }

    #[inline]
    #[must_use]
    pub const fn default_emit_stdout() -> bool {
        true
    }

    #[inline]
    #[must_use]
    pub const fn default_emit_stderr() -> bool {
        false
    }

    #[allow(dead_code)]
    pub fn registry(&self) {
        let Self { file_path, emit_stdout, emit_stderr, log_filters } = self;

        let filter_layer = tracing_subscriber::filter::EnvFilter::new(log_filters.as_str());

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(file_path.clone().map(|path| LogDriver::File(path).layer()))
            .with(emit_stdout.then(|| LogDriver::Stdout.layer()))
            .with(emit_stderr.then(|| LogDriver::Stderr.layer()))
            .init();
    }
}

#[derive(Clone, Debug)]
enum LogDriver {
    Stdout,
    Stderr,
    File(PathBuf),
}

impl LogDriver {
    #[allow(clippy::type_repetition_in_bounds)]
    fn layer<S>(self) -> Option<Box<dyn Layer<S> + Send + Sync + 'static>>
    where
        S: tracing::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        // Shared configuration regardless of where logs are output to.
        let fmt =
            tracing_subscriber::fmt::layer().pretty().with_thread_ids(true).with_thread_names(true);

        // Configure the writer based on the desired log target:
        match self {
            Self::Stdout => Some(Box::new(fmt.with_writer(std::io::stdout))),
            Self::Stderr => Some(Box::new(fmt.with_writer(std::io::stderr))),
            Self::File(path) => {
                let file = OpenOptions::new().create(true).append(true).open(path).ok()?;
                Some(Box::new(fmt.with_writer(file)))
            }
        }
    }
}
