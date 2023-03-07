use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Registry};

use crate::config;

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("Failed to create log directory {0}")]
    DirCreationError(PathBuf),
    #[error("tracing failed to initialized")]
    TracingError,
}

pub fn setup(config: &config::ConfigLog) -> Result<Option<WorkerGuard>> {
    let (file_layer, file_guard) = match &config.location {
        Some(path) => {
            if !path.exists() {
                fs::create_dir(path).with_context(|| LogError::DirCreationError(path.clone()))?;
            }

            let file_appender = tracing_appender::rolling::daily(path, "mediawhaler.log");
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            (
                Some(tracing_subscriber::fmt::layer().with_writer(non_blocking)),
                Some(guard),
            )
        }
        None => (None, None),
    };

    let stdout_layer = match &config.term {
        Some(config::ConfigTerm::StdOut) => {
            Some(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        }
        _ => None,
    };

    let stderr_layer = if let Some(config::ConfigTerm::StdErr) = &config.term {
        Some(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
    } else {
        None
    };

    let subscriber = Registry::default()
        .with(file_layer)
        .with(stdout_layer)
        .with(stderr_layer);

    tracing::subscriber::set_global_default(subscriber).with_context(|| LogError::TracingError)?;

    Ok(file_guard)
}
