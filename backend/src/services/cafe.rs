use crate::models::cafe::Cafe;
use crate::models::cafe::NewCafe;
use crate::models::DbPool;
use actix_web::{error, get, post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use log::debug;

#[get("/")]
pub async fn future_cafes(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let cafes = web::block(move || {
        let mut conn = pool.get().expect("DB connection error");
        Cafe::future_cafes(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let response = HttpResponse::Ok().json(&cafes);

    Ok(response)
}

#[derive(serde::Deserialize)]
pub struct NewCafeData {
    location: String,
    address: String,
    date: String,
}

#[post("/")]
pub async fn create_cafe(
    pool: web::Data<DbPool>,
    form: web::Form<NewCafeData>,
) -> actix_web::Result<impl Responder> {
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
            let new_cafe = NewCafe::new(location, address, date);

            let cafe = web::block(move || {
                let mut conn = pool.get().expect("DB connection error");
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
