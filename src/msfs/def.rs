use msfs::sim_connect::data_definition;
use super::state;

#[data_definition]
#[derive(Debug)]
pub struct Payload {
    #[name = "G FORCE"]
    #[unit = "GForce"]
    pub g_force: f64,
    #[name = "PLANE TOUCHDOWN NORMAL VELOCITY"]
    #[unit = "Degrees"]
    pub touchdown_velocity: f64,
    #[name = "PLANE TOUCHDOWN PITCH DEGREES"]
    #[unit = "Degrees"]
    pub touchdown_pitch_deg: f64,
    #[name = "PLANE TOUCHDOWN HEADING DEGREES MAGNETIC"]
    #[unit = "Degrees"]
    pub touchdown_heading_deg: f64,
    #[name = "PLANE TOUCHDOWN BANK DEGREES"]
    #[unit = "Degrees"]
    pub touchdown_bank_deg: f64,
    #[name = "SIM ON GROUND"]
    #[unit = "Bool"]
    on_ground: bool
}

impl Payload {
    /// Dispatch Landing Rate Notif
    ///     Dispatch a possibility to send a notification based on what has been
    ///     detected in the Payload struct
    pub fn dispatch_landing_rate_notif(&self) {
        if let Ok(mut guard) = state::STATE.lock() {
            guard.set_state(self.on_ground, self.touchdown_velocity);
            guard.send_notification(&self);
        }
    }
}