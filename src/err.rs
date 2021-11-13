use std::fmt;

#[derive(Debug)]
pub enum VSIError {
    TrayItem(String)
}

impl fmt::Display for VSIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VSIError::TrayItem(msg) => write!(f, "Unexpected error when bootstraping the Tray app: {}", msg)
        }
    }
}

impl std::error::Error for VSIError {}
