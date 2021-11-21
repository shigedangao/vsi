use msfs::sim_connect::data_definition;
use super::state;

#[data_definition]
#[derive(Debug, Clone, Default)]
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
    pub on_ground: bool
}

impl Payload {
    /// Dispatch Landing Rate Notif
    ///     Dispatch a possibility to send a notification based on what has been
    ///     detected in the Payload struct
    /// 
    /// # Arguments
    /// 
    /// * `&self` - Self
    pub fn dispatch_landing_rate_notif(&self) {
        if let Ok(mut guard) = state::STATE.lock() {
            guard.set_state(self.on_ground, self.touchdown_velocity);
            guard.send_notification(&self);
        }
    }

    /// Floor Value
    ///     Make the values more digestible for the user
    /// 
    /// # Arguments
    /// * `&mut self` - &Self
    pub fn floor_value(&mut self) -> &Self {
        self.g_force = round_value(self.g_force, 100.0);
        self.touchdown_velocity = round_value(self.touchdown_velocity, 100.0);
        self.touchdown_pitch_deg = round_value(self.touchdown_pitch_deg, 10.0);
        self.touchdown_heading_deg = round_value(self.touchdown_heading_deg, 1.0);
        self.touchdown_bank_deg = round_value(self.touchdown_bank_deg, 10.0);

        self
    }
}

/// Round Value
/// 
/// # Arguments
/// 
/// * `value` - f64
/// * `amount` - f64
fn round_value(value: f64, amount: f64) -> f64 {
    (value * amount).round() / amount
}