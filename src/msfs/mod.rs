use std::pin::Pin;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use native_windows_gui::NoticeSender;
use msfs::sim_connect::{
    Period,
    SimConnect,
    SimConnectRecv,
    SIMCONNECT_OBJECT_ID_USER
};
use msfs::sys::SIMCONNECT_RECV_SIMOBJECT_DATA;
use tracing::{info, warn};
use crate::err::VSIError;

mod def;
mod state;

// Constant
const APP_NAME: &str = "vsi";
const DISPATCH_INTERVAL: u64 = 20;

/// Open Connection
///     Open a connection with SimConnect. If the connection fail
///     we then return an error which will close the thread
fn open_connection<'a>() -> Result<Pin<Box<SimConnect<'a>>>, VSIError> {
    info!(stage = "simconnect", "Opening connection...");

    let sim = SimConnect::open(APP_NAME, handle_recv)
        .map_err(|err| VSIError::SimConnectConnectionFailure(err.to_string()))?;

    info!(stage = "simconnect", "Connection establish");

    Ok(sim)
}

/// Handle Recv
///     Handle fs2020 events return by the SimConnect API
///     /!\ Maybe we could try to throw a panic and see if the thread return an error or else
/// 
/// # Arguments
/// 
/// * `sim` - &mut SimConnect
/// * `recv` - SimConnectRecv
fn handle_recv(sim: &mut SimConnect, recv: SimConnectRecv) {
    match recv {
        SimConnectRecv::SimObjectData(event) => handle_simobject_data_event(sim, event),
        SimConnectRecv::Exception(err) => {
            warn!(stage = "simconnect::handler", "simconnect data error {:?}", err);
            panic!("{:?}", err);
        },
        _ => info!(stage = "simconnect::handler", "not handled event {:?}", recv)
    }
}

/// Handle Simobject Data Event
///     Handle event response of type &SIMCONNECT_RECV_SIMOBJECT_DATA
/// 
/// # Arguments
/// 
/// * `sim` - &mut SimConnect
/// * `event` - &SIMCONNECT_RECV_SIMOBJECT_DATA
fn handle_simobject_data_event(sim: &mut SimConnect, event: &SIMCONNECT_RECV_SIMOBJECT_DATA) {
    match event.dwRequestID {
        0 => {
            if let Some(res) = event.into::<def::Payload>(sim) {
                res.to_owned().floor_value().dispatch_landing_rate_notif();
            }
        },
        _ => {}
    }
}

/// Trigger Simconnect Collection
///     Trigger the simconnect collection within a Thread. The thread is used to not block
///     the System Tray app. Otherwise the user wouldn't be able to interact with the System tray app
///     @link: https://gabdube.github.io/native-windows-gui/native-windows-docs/multithreading.html
///     
///     (!)
///     When an error happened the thread will called the sender which will send a signal to the main thread
///     
/// # Arguments
/// 
/// * `sender` - NoticeSender
pub fn trigger_simconnect_collection(sender: NoticeSender) -> Option<JoinHandle<Result<(), VSIError>>> {
    let handler: JoinHandle<Result<(), VSIError>> = thread::spawn(move || {
        let mut sim = match open_connection() {
            Ok(sim) => sim,
            Err(err) => {
                sender.notice();
                warn!(stage = "simconnect", "Unable to connect to FS2020");
                return Err(err);
            }
        };

        let res = sim.request_data_on_sim_object::<def::Payload>(0, SIMCONNECT_OBJECT_ID_USER, Period::SimFrame);
        if let Err(err) = res {
            sender.notice();
            return Err(VSIError::SimConnectRuntime(err.to_string()));
        }

        // Loop to retrieve the value from the simulator
        // until we're encountering an error
        // A pause is used to not overload the system
        loop {
            match sim.call_dispatch() {
                Ok(_) => thread::sleep(Duration::from_millis(DISPATCH_INTERVAL)),
                Err(err) => {
                    sender.notice();
                    return Err(VSIError::SimConnectRuntime(err.to_string()));
                }
            }
        }
    });

    Some(handler)
}