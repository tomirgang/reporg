use crate::models::user::User;
use crate::permissions::{Role, check_permissions};
use crate::AppState;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::error::{ErrorForbidden, ErrorInternalServerError};
use actix_web::http::header;
use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct BackendUser {
    name: Option<String>,
    email: Option<String>,
    roles: Option<Vec<String>>,
    login_url: Option<String>,
    logout_url: Option<String>,
}

#[get("/")]
async fn user_index(
    user: Option<Identity>,
    session: Session,
) -> Result<impl Responder, actix_web::Error> {
    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Ok().json(BackendUser {
                name: None,
                email: None,
                roles: None,
                login_url: Some("http://127.0.0.1:8000/login/oidc".to_string()),
                logout_url: None,
            }));
        }
    };

    let roles = session.get::<Vec<Role>>("roles")?;
    let roles: Option<Vec<String>> = match roles {
        Some(roles) => Some(
            roles
                .iter()
                .map(|role| match role {
                    Role::Admin => "Admin".to_string(),
                    Role::Organizer => "Organizer".to_string(),
                    Role::Supporter => "Supporter".to_string(),
                    Role::Guest => "Guest".to_string(),
                })
                .collect(),
        ),
        None => None,
    };

    let username = user.id().map_err(|e| ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(BackendUser {
        name: Some(username),
        email: None,
        roles,
        login_url: None,
        logout_url: Some("http://127.0.0.1:8000/logout".to_string()),
    }))
}

#[get("/logout")]
async fn logout(user: Identity, data: web::Data<AppState>) -> impl Responder {
    user.logout();
    HttpResponse::Found()
        .append_header((header::LOCATION, data.url_config.logout_success.clone()))
        .finish()
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
    request: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    info!("Tester login: {:?}", params);

    let api_key = state.settings.tester.key.clone();
    if api_key.len() == 0 {
        log::error!("Invalid TESTER_API_KEY, len is 0!");
        return Err(ErrorForbidden("Invalid TESTER_API_KEY"));
    }

    if params.key != api_key {
        log::error!("Invalid TESTER_API_KEY, wrong key!");
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
        log::error!("Invalid tester role!");
        return Err(ErrorForbidden("Invalid role!"));
    };

    session.insert("roles", vec![role])?;

    log::info!("Tester login successful, using role {:?}", role);

    Identity::login(&request.extensions(), "Tester".into()).unwrap();

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, "/api/user/"))
        .finish())
}

#[derive(Deserialize, Debug)]
pub struct UserList {
    offset: Option<u64>,
    limit: Option<u64>,
}

#[get("/list")]
async fn list(
    state: web::Data<AppState>,
    params: web::Query<UserList>,
    _user: Identity, // require user login
    session: Session,
) -> actix_web::Result<impl Responder> {
    check_permissions(vec![Role::Organizer, Role::Admin, Role::Supporter], session)?;

    let limit = match params.limit {
        Some(l) => l,
        None => 50,
    };

    let offset = match params.offset {
        Some(o) => o,
        None => 0,
    };

    let users = User::page(offset, limit, &state.db).await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(&users))
}
