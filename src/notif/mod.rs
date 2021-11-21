use notify_rust::Notification;
use super::err::VSIError;

/// Notification Manager
///     A struct which helps to operate on the Notification library
pub struct NotificationManager {
    handle: Notification
}

impl NotificationManager {
    /// New
    /// 
    /// # Arguments
    /// 
    /// * `title` - &str
    fn new(title: &str) -> Self {
        let mut handle = Notification::new();
        handle.appname = title.to_string();

        NotificationManager {
            handle
        }
    }

    /// Set Content
    ///     Set optional content
    /// 
    /// # Arguments
    /// 
    /// * `mut self` - self
    /// * `summary` - Option<&str>
    /// * `body` - Option<&str>
    fn set_content(mut self, summary: Option<&str>, body: Option<&str>) -> Self {
        if let Some(content) = summary {
            self.handle.summary(content);
        }

        if let Some(content) = body {
            self.handle.body(content);
        }

        self
    }

    /// Set Sound
    ///     Set the notification sound (use SMS)
    /// 
    /// # Arguments
    /// 
    /// * `mut self` - Self
    fn set_sound(mut self) -> Self {
        self.handle.sound_name("SMS");

        self
    }

    /// Show
    ///     Show the notification. 
    /// 
    /// # Arguments
    /// 
    /// * `&mut self` - Self
    fn show(&mut self) -> Result<(), VSIError> {
        self.handle.show().unwrap();

        Ok(())
    }
}

/// Send Notif
///     Send a notification
/// 
/// # Arguments
/// 
/// * `summary` - Option<&str>
/// * `body` - Option<&str>
pub fn send_notif(summary: Option<&str>, body: Option<&str>) -> Result<(), VSIError> {
    NotificationManager::new("VSI - Landing rate")
        .set_content(summary, body)
        .set_sound()
        .show()
}