extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use color_eyre::eyre::Result;

mod tray;
mod err;
mod notif;
mod msfs;

fn main() -> Result<()> {
    color_eyre::install()?;

    if let Err(err) = tray::bootstrap_tray() {
        panic!("{}", err);
    }

    Ok(())
}
