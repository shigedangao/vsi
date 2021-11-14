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
    println!("youhou");
    // /!\ If we try to establish a connection
    // with the sim closed, the crate won't pick up the connection...
    // a solution would be to maybe check the connection with a timer ?
    // Or use the window crate to get the PID of fs2020...
    let sim = SimConnect::open(APP_NAME, handle_recv)
        .map_err(|err| VSIError::SimConnectConnectionFailure(err.to_string()))?;

    println!("Establish connection with sim");
    Ok(sim)
}

/// Handle Recv
///     Handle fs2020 events return by the SimConnect API
///     /!\ Maybe we could try to throw a panic and see if the thread return an error or else
fn handle_recv(sim: &mut SimConnect, recv: SimConnectRecv) {
    match recv {
        SimConnectRecv::SimObjectData(event) => handle_simobject_data_event(sim, event),
        SimConnectRecv::Exception(err) => println!("{:?}", err),
        _ => println!("{:?}", recv)
    }
}

/// Handle Simobject Data Event
///     Handle event response of type &SIMCONNECT_RECV_SIMOBJECT_DATA
fn handle_simobject_data_event(sim: &mut SimConnect, event: &SIMCONNECT_RECV_SIMOBJECT_DATA) {
    match event.dwRequestID {
        0 => {
            if let Some(res) = event.into::<def::Payload>(sim) {
                res.dispatch_landing_rate_notif();
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
///     The method take a NoticeSender which will signal the main thread
pub fn trigger_simconnect_collection(sender: NoticeSender) -> Option<JoinHandle<Result<(), VSIError>>> {
    let handler: JoinHandle<Result<(), VSIError>> = thread::spawn(move || {
        let mut sim = open_connection()?;
        sim.request_data_on_sim_object::<def::Payload>(0, SIMCONNECT_OBJECT_ID_USER, Period::SimFrame)
            .map_err(|err| VSIError::SimConnectRuntime(err.to_string()))?;

        // Loop until we are encountering an error with the simconnect API
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