#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/pretty_env_logger/0.5.0")]

//! A logger configured via an environment variable which writes to standard
//! error with nice colored output for log levels.
//!
//! ## Example
//!
//! ```
//! extern crate pretty_env_logger;
//! #[macro_use] extern crate log;
//!
//! fn main() {
//!     pretty_env_logger::init();
//!
//!     trace!("a trace example");
//!     debug!("deboogging");
//!     info!("such information");
//!     warn!("o_O");
//!     error!("boom");
//! }
//! ```
//!
//! Run the program with the environment variable `RUST_LOG=trace`.
//!
//! ## Defaults
//!
//! The defaults can be setup by calling `init()` or `try_init()` at the start
//! of the program.
//!
//! ## Enable logging
//!
//! This crate uses [env_logger][] internally, so the same ways of enabling
//! logs through an environment variable are supported.
//!
//! [env_logger]: https://docs.rs/env_logger

#[doc(hidden)]
pub extern crate env_logger;

extern crate log;

use std::fmt::Arguments;

use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder,
};
use log::Level;

/// Initializes the global logger with a pretty env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init() {
    try_init().unwrap();
}

/// Initializes the global logger with a timed pretty env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_timed() {
    try_init_timed().unwrap();
}

/// Initializes the global logger with a pretty env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init() -> Result<(), log::SetLoggerError> {
    try_init_custom_env("RUST_LOG")
}

/// Initializes the global logger with a timed pretty env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_timed() -> Result<(), log::SetLoggerError> {
    try_init_timed_custom_env("RUST_LOG")
}

/// Initialized the global logger with a pretty env logger, with a custom variable name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_custom_env(environment_variable_name: &str) {
    try_init_custom_env(environment_variable_name).unwrap();
}

/// Initialized the global logger with a pretty env logger, with a custom variable name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_builder();

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

/// Initialized the global logger with a timed pretty env logger, with a custom variable name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_timed_custom_env(
    environment_variable_name: &str,
) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_timed_builder();

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

/// Returns a `env_logger::Builder` for further customization.
///
/// This method will return a colored and formatted `env_logger::Builder`
/// for further customization. Refer to env_logger::Build crate documentation
/// for further details and usage.
pub fn formatted_builder() -> Builder {
    let mut builder = Builder::new();

    builder.format(|f, record| {
        use std::io::Write;

        let level = record.level();
        let level_style = level_color(&level);
        let level_value = level_value(&level);

        let target = record.target();
        let target_style = Style::new().bold();

        writeln!(
            f,
            " {}{}{} {}{}{} \n{}{}{}",
            level_style.render(),
            level_value,
            level_style.render_reset(),
            target_style.render(),
            target,
            target_style.render_reset(),
            level_style.render(),
            format_record_args(record.args()),
            level_style.render_reset(),
        )
    });

    builder
}

/// Returns a `env_logger::Builder` for further customization.
///
/// This method will return a colored and time formatted `env_logger::Builder`
/// for further customization. Refer to env_logger::Build crate documentation
/// for further details and usage.
pub fn formatted_timed_builder() -> Builder {
    let mut builder = Builder::new();

    builder.format(|f, record| {
        use std::io::Write;

        let level = record.level();
        let level_style = level_color(&level);
        let level_value = level_value(&level);

        let target = record.target();
        let target_style = Style::new().bold();

        let time = f.timestamp_millis();

        writeln!(
            f,
            " {} {}{}{} {}{}{} \n{}{}{}",
            time,
            level_style.render(),
            level_value,
            level_style.render_reset(),
            target_style.render(),
            target,
            target_style.render_reset(),
            level_style.render(),
            format_record_args(record.args()),
            level_style.render_reset(),
        )
    });

    builder
}

fn level_color(level: &Level) -> Style {
    match level {
        Level::Trace => Style::new().fg_color(Some(AnsiColor::Magenta.into())),
        Level::Debug => Style::new().fg_color(Some(AnsiColor::Blue.into())),
        Level::Info => Style::new().fg_color(Some(AnsiColor::Green.into())),
        Level::Warn => Style::new().fg_color(Some(AnsiColor::Yellow.into())),
        Level::Error => Style::new().fg_color(Some(AnsiColor::Red.into())),
    }
}

fn level_value(level: &Level) -> &'static str {
    match level {
        Level::Trace => "TRACE",
        Level::Debug => "DEBUG",
        Level::Info => "INFO ",
        Level::Warn => "WARN ",
        Level::Error => "ERROR",
    }
}

fn format_record_args(args: &Arguments) -> String {
    let formatted = format!("{}", args);

    // Add 4 spaces to each line
    let indented = formatted
        .lines()
        .map(|line| format!("    {}", line))
        .collect::<Vec<_>>()
        .join("\n");

    indented
}
