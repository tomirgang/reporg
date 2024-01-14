use std::error::Error;
use std::fmt;

use actix_web::ResponseError;
use actix_web::http::StatusCode;

/// A Reporg Error.
#[derive(Debug)]
pub struct ReporgError {
    /// The error description.
    details: String,
    /// The HTTP status code
    status_code: Option<StatusCode>,
}

impl ReporgError {
    /// Create a new error with the given message as description.
    pub fn new(msg: &str, status_code: Option<StatusCode>) -> ReporgError {
        ReporgError{
            details: msg.to_string(),
            status_code,
        }
    }

    /// Transfrom an Error to an ReporgError.
    pub fn from(e: &dyn Error) -> ReporgError {
        ReporgError::new(
            &e.to_string(),
            None,
        )
    }

    /// Transfrom an Error to an ReporgError.
    pub fn from_with_code(e: &dyn Error, status_code: Option<StatusCode>) -> ReporgError {
        ReporgError::new(
            &e.to_string(),
            status_code,
        )
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

impl ResponseError for ReporgError {
    fn status_code(&self) -> StatusCode {
        match self.status_code {
            Some(s) => s,
            None => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
