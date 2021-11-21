use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tracing::error;
use super::def::Payload;
use crate::notif;

// State
//     Global variable used to store the state of each touchdown
//     It would have been better to use a struct which we would pass to 
//     method which need the state. However as we can't control
//     the arguments of simconnect callback we elected to
//     use the global variable solution
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
    /// 
    /// # Arguments
    /// 
    /// * `mut &self` - Self
    /// * `on_ground` - bool
    /// * `touchdown_velocity` - f64
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
    /// 
    /// # Arguments
    /// 
    /// * `&mut self` - self
    /// * `payload` - &Payload
    pub fn send_notification(&mut self, payload: &Payload) {
        // Considering that the aircraft has touchdown
        // The touchdown velocity should be above 0
        // Use a flag to not send an other notification
        if self.on_ground &&
            self.touchdown_velocity > 0.9 &&
            !self.sent {
            // send notif
            let summary = format!("Vertical speed {} fpm", payload.touchdown_velocity);
            let mut content = format!("
                G_Force: {}
                Pitch_attitude: {}
                Heading: {}
                Bank_angle: {}",
                payload.g_force,
                -payload.touchdown_pitch_deg,
                payload.touchdown_heading_deg,
                payload.touchdown_bank_deg
            );

            // Remove whitespace at the beginning of the string
            content = content.replace(" ", "");
            // Add whitespace after the ":" character
            content = content.replace(":", ": ");

            // send the notification
            let res = notif::send_notif(
                Some(summary.as_str()),
                Some(content.as_str())
            );

            if let Err(err) = res {
                error!(state = "touchdown_capture", "unable to send the notification {:?}", err)
            }

            self.sent = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_send_notification() {
        let payload = Payload {
            touchdown_velocity: 100.0,
            on_ground: true,
            ..Default::default()
        };

        let mut state = State::new();
        state.set_state(true, 100.0);
        state.send_notification(&payload);

        assert_eq!(state.sent, true);
    }

    #[test]
    fn expect_to_not_send_notification() {
        let payload = Payload {
            on_ground: false,
            ..Payload::default()
        };
        let mut state = State::new();
        state.set_state(true, 0.0);
        state.send_notification(&payload);

        assert_eq!(state.sent, false);
    }
}