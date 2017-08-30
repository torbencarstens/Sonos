use std;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct SonosError {
    description: String
}

impl SonosError {
    fn new(description: String) -> Self {
        SonosError {
            description: description
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
        self.description.as_str()
    }
}

impl From<std::io::Error> for SonosError {
    fn from(io_error: std::io::Error) -> Self {
        SonosError::new(io_error.description().to_string())
    }
}
