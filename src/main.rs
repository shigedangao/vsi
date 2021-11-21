#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use color_eyre::eyre::Result;
use tracing_subscriber::EnvFilter;
use tracing::error;

mod tray;
mod err;
mod notif;
mod msfs;

/// Prepare Log Lib
///     Only used in the context of dev to get log of the app
fn prepare_log_lib() -> Result<()> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn main() -> Result<()> {
    prepare_log_lib()?;

    if let Err(err) = tray::bootstrap_tray() {
        error!(stage = "init", "message: {}", err);
        panic!("{}", err);
    }

    Ok(())
}
