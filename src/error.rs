use std::fmt;

#[derive(Debug)]
pub enum LaunchdError {
    PathError,
    PlistParseError,
}

impl std::error::Error for LaunchdError {}

impl fmt::Display for LaunchdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LaunchdError::PathError => write!(f, "Failed to get PLIST files"),
            LaunchdError::PlistParseError => write!(f, "Failed to parse PLIST file"),
        }
    }
}
