use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;

#[derive(Default, NwgUi)]
pub struct VsiTray {
    data: RefCell<Option<thread::JoinHandle<u64>>>,

    #[nwg_control]
    #[nwg_events(OnInit: [VsiTray::on_init])]
    window: nwg::MessageWindow,

    #[nwg_control]
    #[nwg_events( OnNotice: [VsiTray::on_notice])]
    notice: nwg::Notice,

    #[nwg_resource(source_file: Some("./plane-blue.ico"))]
    blue_icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.blue_icon), tip: Some("stop"))]
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
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn on_init(&self) {
        let sender = self.notice.sender();
        *self.data.borrow_mut() = Some(thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            println!("on thread");
            1+1
        }));
    }

    fn on_notice(&self) {
        println!("on notice");
    }

    fn connect(&self) {
        nwg::simple_message("Hello", "Hello World!");
        self.tray.set_icon(&self.blue_icon);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub fn bootstrap_ui() {
    nwg::init().expect("Failed to load native gui windows");
    let _ui = VsiTray::build_ui(Default::default()).expect("Failed to build ui");
    nwg::dispatch_thread_events();
}