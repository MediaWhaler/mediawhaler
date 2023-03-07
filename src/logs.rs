use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Layer, Registry};

use crate::config;

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("Failed to create log directory {0}")]
    DirCreationError(PathBuf),
    #[error("tracing failed to initialized")]
    TracingError,
}

pub fn setup(config: &config::ConfigLog) -> Result<Vec<Option<WorkerGuard>>> {
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

    let (output_layer, output_guard) = match &config.term {
        Some(config::ConfigTerm::StdOut) => {
            let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
            (
                Some(
                    tracing_subscriber::fmt::layer()
                        .with_writer(non_blocking)
                        .boxed(),
                ),
                Some(guard),
            )
        }
        Some(config::ConfigTerm::StdErr) => {
            let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stderr());
            (
                Some(
                    tracing_subscriber::fmt::layer()
                        .with_writer(non_blocking)
                        .boxed(),
                ),
                Some(guard),
            )
        }
        _ => (None, None),
    };

    let subscriber = Registry::default()
        .with(file_layer)
        .with(output_layer)
        .with(EnvFilter::from_env("MEDIAWHALER_LOG"));

    tracing::subscriber::set_global_default(subscriber).with_context(|| LogError::TracingError)?;

    Ok(vec![file_guard, output_guard])
}
