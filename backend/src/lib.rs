pub mod models;
pub mod permissions;
pub mod services;
pub mod settings;
pub mod error;
mod migrator;
pub mod entities;
pub mod utils;

use crate::services::cafe::{create_cafe, future_cafes};
use crate::services::login::{logout, oidc_init, oidc_success};
use crate::services::user::{tester_login, user_index, list};
use crate::settings::Settings;
use crate::migrator::Migrator;
use sea_orm_migration::MigratorTrait;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::dev::Server;
use actix_web::{cookie::Key, http, web, App, HttpServer};
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
// using reqwest ends tokio after metadata request :(
use openidconnect::curl::http_client;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlConfig {
    pub login_success: String,
    pub logout_success: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub oidc_client: CoreClient,
    pub db: DatabaseConnection,
    pub url_config: UrlConfig,
    pub settings: Settings,
}

pub async fn run(
    listener: TcpListener,
    redis: RedisSessionStore,
    db: DatabaseConnection,
    settings: Settings,
) -> Server {
    Migrator::refresh(&db).await.unwrap();

    let url_config = UrlConfig {
        login_success: String::from(format!(
            "{}{}",
            &settings.frontend.baseurl, &settings.frontend.login.success
        )),
        logout_success: String::from(format!(
            "{}{}",
            &settings.frontend.baseurl, &settings.frontend.logout.success
        )),
    };

    let oidc_client_id = ClientId::new(settings.oidc.client.id.clone());
    let oidc_client_secret = ClientSecret::new(settings.oidc.client.secret.clone());
    let issuer_url = IssuerUrl::new(settings.oidc.url.issuer.clone()).expect("Invalid issuer URL");
    let redirect_url =
        RedirectUrl::new(settings.oidc.url.redirect.clone()).expect("Invalid redirect URL");

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
            .allowed_origin(&settings.cors.origin)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(redis.clone(), secret_key.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/cafe")
                            .service(future_cafes)
                            .service(create_cafe),
                    )
                    .service(
                        web::scope("/user")
                            .service(user_index)
                            .service(tester_login)
                            .service(list),
                    ),
            )
            .service(logout)
            .service(oidc_init)
            .service(oidc_success)
            .app_data(web::Data::new(AppState {
                oidc_client: client.to_owned(),
                db: db.to_owned(),
                url_config: url_config.to_owned(),
                settings: settings.to_owned(),
            }))
    })
    .listen(listener)
    .expect("Failed to bind to address.")
    .run();

    server
}
