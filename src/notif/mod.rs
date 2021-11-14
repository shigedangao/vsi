use winrt_notification::Toast;
use super::err::VSIError;

pub struct NotificationManager {
    toast: Toast
}

impl NotificationManager {
    fn new(title: &str) -> Self {
        let mut toast = Toast::new(Toast::POWERSHELL_APP_ID);
        toast = toast.title(title);

        NotificationManager {
            toast
        }
    }

    fn set_texts(mut self, first_content: Option<&str>, second_content: Option<&str>) -> Self {
        if let Some(content) = first_content {
            self.toast = self.toast.text1(content);
        }

        if let Some(content) = second_content {
            self.toast = self.toast.text2(content);
        }

        self
    }

    fn show(&mut self) -> Result<(), VSIError> {
        self.toast.show()
            .map_err(VSIError::from)
    }
}

pub fn trigger_demo_notif() -> Result<(), VSIError> {
    NotificationManager::new("VSI")
        .set_texts(Some("hey"), None)
        .show()
}