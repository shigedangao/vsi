use windows::runtime::HSTRING;
use windows::{
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::ToastNotification,
    UI::Notifications::ToastNotificationManager
};
use super::err::VSIError;

pub struct Notification {
    toast_xml: XmlDocument
}

impl Notification {
    fn new_from_template(xml: &str) -> Result<Self, VSIError> {
        let toast_xml = XmlDocument::new()?;
        toast_xml.LoadXml(HSTRING::from(xml))?;

        Ok(Notification {
            toast_xml
        })
    }

    fn create_toast(self) -> Result<(), VSIError> {
        let toast_template = ToastNotification::CreateToastNotification(self.toast_xml)?;
        let toast_notifier = ToastNotificationManager::CreateToastNotifierWithId(HSTRING::from(
            "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe",
        ))?;

        toast_notifier.Show(&toast_template)?;

        Ok(())
    }
}

pub fn test_notification() -> Result<(), VSIError> {
    let xml = r#"
        <toast duration="long">
            <visual>
                <binding template="ToastGeneric">
                    <text id="1">title</text>
                    <text id="2">first line</text>
                    <text id="3">third line</text>
                </binding>
            </visual>
            <audio src="ms-winsoundevent:Notification.SMS" />
            <!-- <audio silent="true" /> -->
        </toast>
    "#;

    Notification::new_from_template(xml)?
        .create_toast()
}