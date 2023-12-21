pub mod models;
pub mod permissions;
pub mod services;

use crate::services::cafe::{create_cafe, future_cafes};
use crate::services::user::{logout, oidc_init, oidc_success, user_index, tester_login};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::dev::Server;
use actix_web::{cookie::Key, http, web, App, HttpServer};
use dotenvy::dotenv;
use models::DbPool;
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::TcpListener;
// using reqwest ends tokio after metadata request :(
use openidconnect::curl::http_client;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlConfig {
    pub login_success: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub oidc_client: CoreClient,
    pub db_pool: DbPool,
    pub url_config: UrlConfig,
}

pub fn run(listener: TcpListener, redis: RedisSessionStore, db_pool: DbPool) -> Server {
    dotenv().ok();

    let url_config = UrlConfig {
        login_success: String::from("/user/"),
    };

    let oidc_client_id = ClientId::new(
        env::var("OIDC_CLIENT_ID").expect("Missing the OIDC_CLIENT_ID environment variable."),
    );
    let oidc_client_secret = ClientSecret::new(
        env::var("OIDC_CLIENT_SECRET")
            .expect("Missing the OIDC_CLIENT_SECRET environment variable."),
    );
    let issuer_url = IssuerUrl::new(
        env::var("OIDC_ISSUER_URL").expect("Missing the OIDC_ISSUER_URL environment variable."),
    )
    .expect("Invalid issuer URL");
    let redirect_url = RedirectUrl::new(
        env::var("OIDC_REDIRECT_URL").expect("Missing the OIDC_REDIRECT_URL environment variable."),
    )
    .expect("Invalid redirect URL");

    let provider_metadata = CoreProviderMetadata::discover(&issuer_url, http_client)
        .expect("Resolving provider metadata failed!");

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        oidc_client_id,
        Some(oidc_client_secret),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(redirect_url);

    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(redis.clone(), secret_key.clone()))
            .service(
                web::scope("/cafe")
                    .service(future_cafes)
                    .service(create_cafe),
            )
            .service(
                web::scope("/user")
                    .service(user_index)
                    .service(logout)
                    .service(oidc_init)
                    .service(oidc_success)
                    .service(tester_login),
            )
            .app_data(web::Data::new(AppState {
                oidc_client: client.to_owned(),
                db_pool: db_pool.to_owned(),
                url_config: url_config.to_owned(),
            }))
    })
    .listen(listener)
    .expect("Failed to bind to address.")
    .run();

    server
}
