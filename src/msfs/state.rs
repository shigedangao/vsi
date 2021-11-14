use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use super::def::Payload;
use crate::notif;

lazy_static! {
    pub static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
}

/// State
///     State used as a Global variable to detect
///     whenever we need to send a Notification
///     /!\ It would have been better if we could pass the state w/o having a global var
///         However, we are not able to add additional parameter to the callback method of SimConnect
#[derive(Debug, Default)]
pub struct State {
    touchdown_velocity: f64,
    on_ground: bool,
    sent: bool
}

impl State {
    /// New
    ///     Create a new state
    pub fn new() -> Self {
        State::default()
    }

    /// Set State
    ///     Set the state which'll be used to check whenever
    ///     sending a notification
    pub fn set_state(&mut self, on_ground: bool, touchdown_velocity: f64) {
        // The aircraft could have bounced
        // So we are setting the sent flag to false in this case
        // in order to send an other notification if needed
        if !on_ground {
            self.sent = false;
        }

        self.touchdown_velocity = touchdown_velocity;
        self.on_ground = on_ground;
    }

    /// Send Notification
    ///     Send a notification by using the notif mod
    pub fn send_notification(&mut self, payload: &Payload) {
        // Considering that the aircraft has touchdown
        // The touchdown velocity should be above 0
        // Use a flag to not send an other notification
        if self.on_ground &&
            self.touchdown_velocity > 0.9 &&
            !self.sent {
            // send notif
            let res = notif::trigger_demo_notif(
                Some(format!(
                    "Vertical Speed: {} fpm / G Force: {}",
                    payload.touchdown_velocity,
                    payload.g_force
                ).as_str()),
                Some(format!(
                    "Pitch attitude {} / Heading {} / Bank angle {}",
                    payload.touchdown_pitch_deg,
                    payload.touchdown_heading_deg,
                    payload.touchdown_bank_deg
                ).as_str())
            );

            if let Err(err) = res {
                // log the error
                // @TODO use real logger
                println!("{:?}", err);
            }

            self.sent = true;
        }
    }
}