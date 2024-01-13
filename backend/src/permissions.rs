use actix_session::Session;
use actix_web::error::ErrorForbidden;
use actix_web::Error;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::models::user::{User, Roles};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum Role {
    Guest = 1,
    Supporter = 2,
    Organizer = 4,
    Admin = 8,
}

pub fn check_permissions(required: i32, session: Session) -> Result<(), Error> {
    let user = session
        .get::<User>("user")?
        .ok_or_else(|| ErrorForbidden("No user found!"))?;
    let roles = user.get_roles();

    debug!(
        "Check permissions: required: {:?}, available: {:?}",
        required, roles
    );

    if roles & required > 0 {
        return Ok(())
    } else {
        Err(ErrorForbidden("Required role not found!"))
    }
}
