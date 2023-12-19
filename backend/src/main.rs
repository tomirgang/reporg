use std::net::TcpListener;
use backend::models::establish_connection;
use backend::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let connection_pool = establish_connection();
    let address = format!("127.0.0.1:8000");
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

