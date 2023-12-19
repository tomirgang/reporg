use crate::models::cafe::Cafe;
use crate::models::DbPool;
use actix_web::{error, get, web, HttpResponse, Responder};

#[get("/")]
pub async fn future_cafes(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let cafes = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("DB connection error");
        Cafe::future_cafes(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let response = HttpResponse::Ok().json(&cafes);
    
    Ok(response)
}
