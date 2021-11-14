use std::fmt;

#[derive(Debug)]
pub enum VSIError {
    TrayBootload(String),
    Notif(String),
    SimConnectConnectionFailure(String),
    SimConnectRuntime(String)
}

impl fmt::Display for VSIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VSIError::TrayBootload(msg) => write!(f, "Unexpected error when bootstraping the Tray app: {}", msg),
            VSIError::Notif(msg) => write!(f, "Unexpected error while triggering the notif {}", msg),
            VSIError::SimConnectConnectionFailure(msg) => write!(f, "An error happened when trying to establish connection with SimConnect {}", msg),
            VSIError::SimConnectRuntime(msg) => write!(f, "Unexpected error while communicating with SimConnect {}", msg)
        }
    }
}

impl std::error::Error for VSIError {}

impl From<nwg::NwgError> for VSIError {
    fn from(err: nwg::NwgError) -> Self {
        VSIError::TrayBootload(err.to_string()) 
    }
}

impl From<winrt_notification::Error> for VSIError {
    fn from(err: winrt_notification::Error) -> Self {
        VSIError::Notif(err.message())
    }
}