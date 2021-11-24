use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;
use std::thread;
use super::err::VSIError;
use crate::msfs;
use crate::notif;
use tracing::{info, warn};

/// Vsi Tray
///     Struct used to create a tray app icon
#[derive(Default, NwgUi)]
pub struct VsiTray {
    data: RefCell<Option<thread::JoinHandle<Result<(), VSIError>>>>,

    #[nwg_control]
    #[nwg_events(OnInit: [VsiTray::on_init])]
    window: nwg::MessageWindow,

    #[nwg_control]
    #[nwg_events( OnNotice: [VsiTray::on_notice])]
    notice: nwg::Notice,

    #[nwg_resource(source_bin: Some(include_bytes!("../plane-blue.ico")))]
    blue_icon: nwg::Icon,

    #[nwg_resource(source_bin: Some(include_bytes!("../plane-red.ico")))]
    red_icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.red_icon), tip: Some("VSI FS2020 monitor"))]
    #[nwg_events(MousePressLeftUp: [VsiTray::show_menu], OnContextMenu: [VsiTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Re-connect to FS2020")]
    #[nwg_events(OnMenuItemSelected: [VsiTray::connect])]
    tray_item_reconnect: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [VsiTray::exit])]
    tray_item_exit: nwg::MenuItem,

}

impl VsiTray {
    /// Show Menu
    /// 
    /// # Arguments
    /// 
    /// * `&self` - &VsiTray
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    /// On Init
    /// 
    /// # Arguments
    /// 
    /// * `&self` - &VsiTray
    fn on_init(&self) {
        connect_to_sim(&self);
    }

    /// On Notice
    ///     Send a notification that the app wasn't able to connect to FS2020
    ///     
    /// # Arguments
    /// 
    /// * `&self` - &VsiTray
    fn on_notice(&self) {
        match notif::send_notif(
            Some("Unable to connec to FS2020. Please click on reconnect when the simulator is available"), 
            None
        ) {
            Ok(_) => info!(step = "notice", "Unable to connect notif sent"),
            Err(err) => warn!(step = "notice", "Unable to send notif {}", err)
        };

        self.tray.set_icon(&self.red_icon);
    }

    /// Connect
    ///     Click on the connect button to try to re-establish a connection with FS2020
    /// 
    /// # Arguments
    /// 
    /// * `&self` - &VsiTray
    fn connect(&self) {
        connect_to_sim(&self);
    }

    /// Exit
    ///     Exit the app
    /// 
    /// # Arguments
    /// 
    /// * `&self` - &VsiTray
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

/// Connect To Sim
///     Connect to the simulator and set the icon
///     Connection to the simulator happened asynchronously within a thread. See the definition of the 
///     trigger_simconnect_collection method for more information
/// 
/// # Arguments
/// 
/// * `vsi` - &VsiTray
fn connect_to_sim(vsi: &VsiTray) {
    let sender = vsi.notice.sender();
    *vsi.data.borrow_mut() = msfs::trigger_simconnect_collection(sender);
    vsi.tray.set_icon(&vsi.blue_icon);
}

/// Boostrap Tray
///     Run the tray app
pub fn bootstrap_tray() -> Result<(), VSIError> {
    nwg::init()?;
    let _ui = VsiTray::build_ui(Default::default())?;
    nwg::dispatch_thread_events();

    Ok(())
}