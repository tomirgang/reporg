use crate::components::nav::Nav;
use crate::views::{
    contact::Contact, home::Home, members::one::One, members::three::Three, members::two::Two,
    members::Members, not_found::NotFound, register::Register,
};
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/anmelden")]
    Register,
    #[at("/kontakt")]
    Contact,
    #[at("/mitarbeiter/*")]
    Members,
    #[at("/mitarbeiter")]
    MembersRoot,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum MembersRoute {
    #[at("/mitarbeiter/one")]
    One,
    #[at("/mitarbeiter/two")]
    Two,
    #[at("/mitarbeiter/three")]
    Three,
    #[not_found]
    #[at("/mitarbeiter")]
    Members,
}

fn switch_members(route: MembersRoute) -> Html {
    match route {
        MembersRoute::Members => html! { <Members /> },
        MembersRoute::One => html! { <One /> },
        MembersRoute::Two => html! { <Two /> },
        MembersRoute::Three => html! { <Three /> },
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Register => html! { <Register /> },
        Route::Contact => html! { <Contact /> },
        Route::Members | Route::MembersRoot => {
            html! { <Switch<MembersRoute> render={switch_members} /> }
        }
        Route::NotFound => html! { <NotFound /> },
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    Guest,
    Supporter,
    Organizer,
    Admin,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserData {
    pub name: String,
    pub roles: Vec<Role>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct BackendUser {
    name: Option<String>,
    email: Option<String>,
    roles: Option<Vec<String>>,
    login_url: Option<String>,
    logout_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub user: Option<UserData>,
    pub login_url: Option<String>,
    pub logout_url: Option<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState {
        user: None,
        login_url: None,
        logout_url: None,
    });
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let user = Request::get("/api/user/").send().await;

                let response = match user {
                    Ok(user) => user,
                    Err(e) => {
                        log::error!("User request failed: {}", e);
                        return;
                    }
                };

                let backend_user: BackendUser = match response.json().await {
                    Ok(user) => user,
                    Err(e) => {
                        log::error!("User request failed: {}", e);
                        return;
                    }
                };

                let roles: Vec<Role> = match backend_user.roles {
                    Some(roles) => roles
                        .iter()
                        .map(|role| {
                            if role == "Admin" {
                                Role::Admin
                            } else if role == "Organizer" {
                                Role::Organizer
                            } else if role == "Supporter" {
                                Role::Supporter
                            } else {
                                Role::Guest
                            }
                        })
                        .collect(),
                    None => vec![],
                };

                let user = match backend_user.name {
                    Some(name) => Some(UserData { name, roles }),
                    None => None,
                };

                state.set(AppState {
                    user,
                    login_url: backend_user.login_url,
                    logout_url: backend_user.logout_url,
                    ..(*state).clone()
                });
            });
            || ()
        });
    }

    html! {
        <ContextProvider<AppState> context={(*state).clone()}>
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </ContextProvider<AppState>>
    }
}
