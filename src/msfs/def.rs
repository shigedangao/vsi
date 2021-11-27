use msfs::sim_connect::data_definition;
use super::state;

#[data_definition]
#[derive(Debug, Clone, Default)]
pub struct Payload {
    #[name = "G FORCE"]
    #[unit = "GForce"]
    pub g_force: f64,
    #[name = "VERTICAL SPEED"]
    #[unit = "Feet"]
    pub vertical_speed: f64,
    #[name = "PLANE TOUCHDOWN PITCH DEGREES"]
    #[unit = "Degrees"]
    pub touchdown_pitch_deg: f64,
    #[name = "PLANE TOUCHDOWN HEADING DEGREES MAGNETIC"]
    #[unit = "Degrees"]
    pub touchdown_heading_deg: f64,
    #[name = "PLANE TOUCHDOWN BANK DEGREES"]
    #[unit = "Degrees"]
    pub touchdown_bank_deg: f64,
    #[name = "AIRSPEED INDICATED"]
    #[unit = "Knots"]
    pub indicated_airspeed: f64,
    #[name = "AMBIENT WIND DIRECTION"]
    #[unit = "Degrees"]
    pub wind_direction: f64,
    #[name = "AMBIENT WIND VELOCITY"]
    #[unit = "Knots"]
    pub wind_velocity: f64,
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
            guard.set_state(self.on_ground, self.g_force);
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
        // Vertical speed return a value per second
        self.vertical_speed = round_value(self.vertical_speed * 60.0, 100.0);
        // Sim seems to return a negative value. Hence we're making it positive (usually we don't touchdown nose first)
        self.touchdown_pitch_deg = - round_value(self.touchdown_pitch_deg, 10.0);
        self.touchdown_heading_deg = round_value(self.touchdown_heading_deg, 1.0);
        self.touchdown_bank_deg = round_value(self.touchdown_bank_deg, 10.0);
        self.indicated_airspeed = round_value(self.indicated_airspeed, 1.0);
        self.wind_direction = round_value(self.wind_direction, 1.0);
        self.wind_velocity = round_value(self.wind_velocity, 1.0);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_round_values() {
        let mut payload = Payload {
            g_force: 1.1254687,
            vertical_speed: -1.0,
            touchdown_pitch_deg: -4.2555,
            touchdown_heading_deg: 123.2545,
            touchdown_bank_deg: 1.245468,
            indicated_airspeed: 125.2,
            wind_direction: 340.5,
            wind_velocity: 10.0,
            on_ground: true,
        };

        payload.floor_value();

        assert_eq!(payload.g_force, 1.13);
        assert_eq!(payload.vertical_speed, -60.0);
        assert_eq!(payload.touchdown_pitch_deg, 4.3);
        assert_eq!(payload.touchdown_heading_deg, 123.0);
        assert_eq!(payload.touchdown_bank_deg, 1.2);
        assert_eq!(payload.indicated_airspeed, 125.0);
        assert_eq!(payload.wind_direction, 341.0);
        assert_eq!(payload.wind_velocity, 10.0);
    }
}