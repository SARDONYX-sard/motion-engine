//! Initializing a multithreaded logger in Rust.
//!
//! This library provides functionality to initialize a multithreaded logger with flexible configuration options.
//! It supports both file I/O logging and logging to stdout with ANSI color support.
//!
//! # Examples
//!
//! ```
//! use anyhow::Result;
//! use tracing::{level_filters::LevelFilter, subscriber::DefaultGuard};
//! use my_logger::init_tracing;
//!
//! fn main() -> Result<()> {
//!     let (worker_guard, default_guard) = init_tracing(Some("test_log"), true, LevelFilter::INFO)?;
//!     // Your application code here
//!     Ok(())
//! }
//! ```
//!
//! # Arguments
//!
//! * `test_name`: An optional `&str` representing the name of the test. If `None`, only stdout logging is performed.
//! * `with_stdio`: A `bool` indicating whether to log to stdout with ANSI color support.
//! * `filter`: An implementation of `Into<LevelFilter>` specifying the logging level filter.
//!
//! # Returns
//!
//! Returns a tuple containing guards:
//! * `worker_guard`: An optional `WorkerGuard` for file logging. If dropped, the file logging stops.
//! * `default_guard`: A `DefaultGuard` for controlling the logger. If dropped, the logger stops.
//!
//! # Errors
//!
//! Returns an `Err` if any error occurs during logger initialization.
//!
//! # Panics
//!
//! Panics if logging to a file fails or if setting the logger fails.
//!
//! # Notes
//!
//! * File I/O logging does not support ANSI color.
//! * Logging to stdout supports ANSI color.
//! * If `test_name` is `None`, only stdout logging is performed.
//! * Dropping `worker_guard` stops file logging.
//! * Dropping `default_guard` stops the logger.
//!
use anyhow::Result;
use tracing::{level_filters::LevelFilter, subscriber::DefaultGuard};
use tracing_appender::non_blocking::WorkerGuard;

/// Multithread init logger.
///
/// File I/O is No ANSI color, output to stdout has ANSI color.
/// - `test_name`: If [`None`] then stdout only prints.
///
/// # Returns
/// Guards
/// - If this variable is dropped, the logger stops.
pub fn init_tracing(
    test_name: Option<&str>,
    with_stdio: bool,
    filter: impl Into<LevelFilter>,
) -> Result<(Option<WorkerGuard>, DefaultGuard)> {
    use tracing_subscriber::{fmt, layer::SubscriberExt};

    let (worker_guard, default_guard) = match test_name {
        Some(test_name) => {
            std::fs::create_dir_all("../../logs")?;
            let (file_writer, guard) = tracing_appender::non_blocking(std::fs::File::create(
                format!("../../logs/{test_name}.log"),
            )?);

            let thread_guard = match with_stdio {
                true => tracing::subscriber::set_default(
                    fmt::Subscriber::builder()
                        .compact()
                        .pretty()
                        .with_file(true)
                        .with_line_number(true)
                        .with_max_level(filter)
                        .with_target(false)
                        .finish()
                        .with(
                            fmt::Layer::default()
                                .compact()
                                .with_ansi(false)
                                .with_file(true)
                                .with_line_number(true)
                                .with_target(false)
                                .with_writer(file_writer),
                        ),
                ),
                false => tracing::subscriber::set_default(
                    fmt::Subscriber::builder()
                        .compact()
                        .with_ansi(false)
                        .with_file(true)
                        .with_line_number(true)
                        .with_target(false)
                        .with_writer(file_writer)
                        .with_max_level(filter)
                        .finish(),
                ),
            };

            (Some(guard), thread_guard)
        }
        None => (
            None,
            tracing::subscriber::set_default(
                fmt::Subscriber::builder()
                    .compact()
                    .pretty()
                    .with_file(true)
                    .with_line_number(true)
                    .with_max_level(filter)
                    .with_target(false)
                    .finish(),
            ),
        ),
    };
    Ok((worker_guard, default_guard))
}
