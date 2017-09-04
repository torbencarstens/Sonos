use std;
use std::error::Error;
use std::fmt;

use super::ErrorKind;

#[derive(Debug)]
pub struct SonosError {
    error_kind: ErrorKind
}

impl SonosError {
    pub fn new(error_kind: ErrorKind) -> Self {
        SonosError {
            error_kind: error_kind
        }
    }
}

impl std::fmt::Display for SonosError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for SonosError {
    fn description(&self) -> &str {
        match self.error_kind {
            ErrorKind::DiscoveryError(ref description) => description.as_str()
        }
    }
}

// From<std::io::Error> only happens when receiving it via `sonos_discovery`.
// -> We can just use `DiscoveryError` here.
impl From<std::io::Error> for SonosError {
    fn from(io_error: std::io::Error) -> Self {
        SonosError::new(ErrorKind::DiscoveryError(io_error.description().to_string()))
    }
}
