use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct TeleportError {
    details: String,
}

impl TeleportError {
    pub fn new(msg: &str) -> TeleportError {
        TeleportError {
            details: msg.to_string(),
        }
    }
}

impl Display for TeleportError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TeleportError {
    fn description(&self) -> &str {
        &self.details
    }
}
