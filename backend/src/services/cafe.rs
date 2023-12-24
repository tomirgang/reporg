use crate::models::cafe::Cafe;
use crate::models::cafe::NewCafe;
use crate::permissions::{check_permissions, Role};
use crate::AppState;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{error, get, post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use log::debug;

#[get("/")]
pub async fn future_cafes(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let cafes = web::block(move || {
        let mut conn = state.db_pool.get().expect("DB connection error");
        Cafe::future_cafes(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&cafes))
}

#[derive(serde::Deserialize)]
pub struct NewCafeData {
    location: String,
    address: String,
    date: String,
}

#[post("/")]
pub async fn create_cafe(
    state: web::Data<AppState>,
    form: web::Form<NewCafeData>,
    _user: Identity, // require user login
    session: Session,
) -> actix_web::Result<impl Responder> {
    check_permissions(vec![Role::Organizer, Role::Admin], session)?;

    let actix_web::web::Form(NewCafeData {
        location,
        address,
        date,
    }) = form;

    debug!(
        "POST: create date - location: {}, address: {}, date: {}",
        &location, &address, &date
    );

    match NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M") {
        Ok(date) => {
            let new_cafe = NewCafe::new(&location, &address, date);

            let cafe = web::block(move || {
                let mut conn = state.db_pool.get().expect("DB connection error");
                new_cafe.save(&mut conn)
            })
            .await?
            .map_err(error::ErrorInternalServerError)?;

            let response = HttpResponse::Ok().json(&cafe);

            Ok(response)
        }
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}
