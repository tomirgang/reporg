use actix_session::Session;
use actix_web::error::ErrorForbidden;
use actix_web::Error;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum Role {
    Guest,
    Supporter,
    Organizer,
    Admin,
}

pub fn check_permissions(required: Vec<Role>, session: Session) -> Result<(), Error> {
    let roles = session
        .get::<Vec<Role>>("roles")?
        .ok_or_else(|| ErrorForbidden("No roles found!"))?;

    debug!(
        "Check permissions: required: {:?}, available: {:?}",
        required, roles
    );

    for role in required.iter() {
        if roles.iter().any(|&r| r == *role) {
            return Ok(());
        }
    }

    Err(ErrorForbidden("Required role not found!"))
}
