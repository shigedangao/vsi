extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

mod tray;
mod err;

fn main() {
    tray::bootstrap_ui();
}
