use std::error::Error;
use std::fmt;

/// A Reporg Error.
#[derive(Debug)]
pub struct ReporgError {
    /// The error description.
    details: String
}

impl ReporgError {
    
    /// Create a new error with the given message as description.
    pub fn new(msg: &str) -> ReporgError {
        ReporgError{details: msg.to_string()}
    }

    /// Transfrom an Error to an ReporgError.
    pub fn from(e: &dyn Error) -> ReporgError {
        ReporgError::new(&e.to_string())
    }
}

impl fmt::Display for ReporgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ReporgError {
    fn description(&self) -> &str {
        &self.details
    }
}
