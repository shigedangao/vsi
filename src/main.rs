extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
extern crate winrt_notification;

use color_eyre::eyre::Result;

mod tray;
mod err;
mod notif;

fn main() -> Result<()> {
    color_eyre::install()?;
    notif::utils::set_app_model_id();
    if let Err(err) = tray::bootstrap_tray() {
        panic!("{}", err);
    }

    Ok(())
}
