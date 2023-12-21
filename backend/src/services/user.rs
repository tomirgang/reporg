use crate::AppState;
use crate::permissions::Role;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::error::{ErrorBadRequest, ErrorForbidden};
use actix_web::http::header;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::info;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AccessTokenHash, AuthorizationCode, CsrfToken, Nonce, PkceCodeChallenge, PkceCodeVerifier,
};
use openidconnect::{OAuth2TokenResponse, TokenResponse};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct OidcSecrets {
    pkce_verifier: PkceCodeVerifier,
    csrf_token: CsrfToken,
    nonce: Nonce,
}

#[get("/login/oidc")]
async fn oidc_init(
    data: web::Data<AppState>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = data
        .oidc_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .set_pkce_challenge(pkce_challenge)
        .url();

    session.insert(
        "oidc_secrets",
        OidcSecrets {
            pkce_verifier,
            csrf_token,
            nonce,
        },
    )?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[get("/login/oidc/success")]
async fn oidc_success(
    request: HttpRequest,
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());

    let secrets = match session.get::<OidcSecrets>("oidc_secrets")? {
        Some(secrets) => secrets,
        None => return Err(ErrorBadRequest("No OIDC secrets found!")),
    };

    if state.secret() != secrets.csrf_token.secret() {
        return Err(ErrorBadRequest("CSRF token error!"));
    }

    let token_response = data
        .oidc_client
        .exchange_code(code)
        .set_pkce_verifier(secrets.pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|e| ErrorBadRequest(e))?;

    let id_token = token_response
        .id_token()
        .ok_or_else(|| return ErrorBadRequest("OIDC Server did not return an ID token"))?;

    let claims = id_token
        .claims(&data.oidc_client.id_token_verifier(), &secrets.nonce)
        .map_err(|e| ErrorBadRequest(e))?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            &id_token.signing_alg().map_err(|e| ErrorBadRequest(e))?,
        )
        .map_err(|e| ErrorBadRequest(e))?;

        if actual_access_token_hash != *expected_access_token_hash {
            return Err(ErrorBadRequest("Invalid access token"));
        }
    }

    let username = claims
        .preferred_username()
        .map(|username| username.as_str())
        .ok_or_else(|| ErrorBadRequest("Providing a preferred username is mandatory!"))?;
    let _email = claims
        .email()
        .map(|email| email.as_str())
        .ok_or_else(|| ErrorBadRequest("Providing an e-mail address is mandatory!"))?;

    // TODO: create user

    Identity::login(&request.extensions(), username.into()).unwrap();

    session.insert("roles", vec!{Role::Supporter})?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, data.url_config.login_success.clone()))
        .finish())
}

#[get("/")]
async fn user_index(user: Option<Identity>, session: Session) -> Result<impl Responder, actix_web::Error> {
    let roles = session.get::<Vec<Role>>("roles")?;

    if let Some(user) = user {
        Ok(format!("Welcome {}!<br/>Roles: {:?}", user.id().unwrap(), roles))
    } else {
        Ok("Welcome Anonymous!".to_owned())
    }
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

#[derive(Deserialize, Debug)]
pub struct TesterLogin {
    role: String,
    key: String,
}

#[get("/tester_login")]
async fn tester_login(
    session: Session,
    params: web::Query<TesterLogin>,
    data: web::Data<AppState>,
    request: HttpRequest,
) -> Result<impl Responder, actix_web::Error> {

    info!("Tester login: {:?}", params);

    dotenv().ok();

    let api_key = env::var("TESTER_API_KEY").map_err(|e| ErrorForbidden(e))?;

    if params.key != api_key {
        return Err(ErrorForbidden("Invalid API key"));
    }
    
    let role = if params.role == "Admin" {
        Role::Admin
    } else if params.role == "Organizer" {
        Role::Organizer
    } else if params.role == "Supporter" {
        Role::Supporter
    } else if params.role == "Guest" {
        Role::Guest
    } else {
        return Err(ErrorForbidden("Invalid role!"));
    };

    session.insert("roles", vec!{role})?;

    Identity::login(&request.extensions(), "Tester".into()).unwrap();

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, data.url_config.login_success.clone()))
        .finish())
}
