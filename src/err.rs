use std::fmt::{self, write};

#[derive(Debug)]
pub enum VSIError {
    TrayBootload(String),
    Xml(String),
    WindowAppInfo(String)
}

impl fmt::Display for VSIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VSIError::TrayBootload(msg) => write!(f, "Unexpected error when bootstraping the Tray app: {}", msg),
            VSIError::Xml(msg) => write!(f, "Unexpected error while loading the XML {}", msg),
            VSIError::WindowAppInfo(msg) => write!(f, "Unable to retrieve app info {}", msg)
        }
    }
}

impl std::error::Error for VSIError {}

impl From<nwg::NwgError> for VSIError {
    fn from(err: nwg::NwgError) -> Self {
        VSIError::TrayBootload(err.to_string()) 
    }
}

impl From<windows::runtime::Error> for VSIError {
    fn from(err: windows::runtime::Error) -> Self {
        VSIError::Xml(err.message().to_string())
    }
}