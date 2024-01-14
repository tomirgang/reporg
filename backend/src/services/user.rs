use crate::models::user::{User, NewUser, Roles};
use crate::permissions::{Role, check_permissions};
use crate::AppState;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::error::{ErrorForbidden, ErrorNotFound};
use actix_web::http::header;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct BackendUser {
    name: Option<String>,
    email: Option<String>,
    roles: i32,
    login_url: Option<String>,
    logout_url: Option<String>,
}

impl BackendUser {
    fn new() -> BackendUser {
        BackendUser {
            name: None,
            email: None,
            roles: 0,
            login_url: None,
            logout_url: None,
        }
    }

    fn from(u: &User) -> BackendUser {
        BackendUser {
            name: Some(u.name.clone()),
            email: Some(u.email.clone()),
            roles: u.roles,
            login_url: None,
            logout_url: None,
        }
    }
}

#[get("/")]
async fn user_index(
    _identity: Option<Identity>,
    session: Session,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let u = match session.get::<User>("user")? {
        Some(u) => {
            let url = format!("{}/logout", state.settings.backend.baseurl);
            let mut u = BackendUser::from(&u);
            u.logout_url = Some(url);
            u
        },
        None => {
            let url = format!("{}/login/oidc", state.settings.backend.baseurl);
            let mut u = BackendUser::new();
            u.login_url = Some(url);
            u
        }
    };

    Ok(HttpResponse::Ok().json(u))
}

#[get("/logout")]
async fn logout(
    identity: Identity,
    session: Session,
    state: web::Data<AppState>
) -> impl Responder {
    identity.logout();
    session.clear();

    HttpResponse::Found()
        .append_header((header::LOCATION, state.url_config.logout_success.clone()))
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

    let u = User::find_by_email("tester@example.com", &state.db).await?;

    let u = match u {
        Some(mut u) => {
            u.set_roles(role as i32);
            u.save(&state.db).await?
        },
        None => {
            let mut new_user = NewUser::new("Tester", "tester@example.com", None);
            new_user.set_roles(role as i32);
            new_user.save(&state.db).await?
        }
    };
    
    Identity::login(&request.extensions(), u.email.clone())?;

    session.insert("user", u)?;

    log::info!("Tester login successful, using role {:?}", role);

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, "/api/user/"))
        .finish())
}

#[derive(Deserialize, Debug)]
pub struct UserList {
    offset: Option<u64>,
    limit: Option<u64>,
    roles: Option<i32>,
}

#[get("/list")]
async fn list(
    state: web::Data<AppState>,
    params: web::Query<UserList>,
    _user: Identity, // require user login
    session: Session,
) -> actix_web::Result<impl Responder> {
    check_permissions(Role::Organizer as i32 | Role::Admin as i32, session)?;

    let limit = match params.limit {
        Some(l) => l,
        None => 50,
    };

    let offset = match params.offset {
        Some(o) => o,
        None => 0,
    };

    let roles = match params.roles {
        Some(r) => Some(r),
        None => None,
    };

    let users = User::page(offset, limit, &state.db, roles).await?;

    Ok(HttpResponse::Ok().json(&users))
}

#[derive(Deserialize, Debug)]
pub struct UserDetail {
    pub user_id: i32,
}

#[get("/{user_id}")]
async fn user(
    state: web::Data<AppState>,
    path: web::Path<UserDetail>,
    _user: Identity, // require user login
    session: Session,
) -> actix_web::Result<impl Responder> {
    // get user
    let user = session.get::<User>("user")?;
    let user = match user {
        Some(u) => u,
        None => return Err(ErrorForbidden("No user found!")),
    };
    // check if user is allowed to view the details
    let allowed = user.id == path.user_id;
    if !allowed {
        match check_permissions(Role::Organizer as i32 | Role::Admin as i32, session) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
    }

    let user = User::find(path.user_id, &state.db).await?;

    match user {
        Some(u) => Ok(HttpResponse::Ok().json(&u)),
        None => Err(ErrorNotFound("No user with this ID!")),
    }
}

#[post("/update")]
pub async fn create_cafe(
    state: web::Data<AppState>,
    user_data: web::Form<User>,
    _user: Identity, // require user login
    session: Session,
) -> actix_web::Result<impl Responder> {
    let actix_web::web::Form(user_data) = user_data;

    // get user
    let u = session.get::<User>("user")?;
    let u = match u {
        Some(u) => u,
        None => return Err(ErrorForbidden("No user found!")),
    };
    // check if user is allowed to view the details
    let allowed = u.id == user_data.id;
    if !allowed {
        match check_permissions(Role::Organizer as i32 | Role::Admin as i32, session) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
    }
    // get old user data
    let old_user = User::find(user_data.id, &state.db).await?;
    let old_user = match old_user {
        Some(o) => o,
        None => return Err(ErrorForbidden("No user found!")),
    };
    // check if changes are allowed
    if user_data.email != old_user.email {
        return Err(ErrorForbidden("Email is not allowed to change!"));
    }
    if user_data.roles != old_user.roles  && !u.is_admin(){
        return Err(ErrorForbidden("Only admins can change user roles!"));
    }

    log::debug!("[user.update] new user data: {:?}", &user_data);

    let modified_user = user_data.save(&state.db).await?;
    
    Ok(HttpResponse::Ok().json(modified_user))
}

#[get("/{user_id}/delete")]
async fn delete(
    state: web::Data<AppState>,
    path: web::Path<UserDetail>,
    _user: Identity, // require user login
    session: Session,
) -> actix_web::Result<impl Responder> {
    // get user
    let u = session.get::<User>("user")?;
    let u = match u {
        Some(u) => u,
        None => return Err(ErrorForbidden("No user found!")),
    };
    if u.id != path.user_id {
        return Err(ErrorForbidden("No sufficent permissions!"));
    }

    let _result = u.delete(&state.db).await?;

    Ok(HttpResponse::Ok())
}
