use crate::app::AppState;
use crate::app::Role;
use yew::prelude::*;

#[function_component(User)]
pub fn user() -> Html {
    let state = use_context::<AppState>();
    let (user, login_url, logout_url) = match state {
        Some(state) => {
            let AppState {
                user,
                login_url,
                logout_url,
            } = state;
            (user, login_url, logout_url)
        }
        None => (None, None, None),
    };

    if let Some(user) = user {
        let roles: Vec<Html> = user
            .roles
            .iter()
            .map(|role| {
                let role = match role {
                    Role::Admin => "Administrator".to_string(),
                    Role::Organizer => "Organisator".to_string(),
                    Role::Supporter => "Helfer".to_string(),
                    Role::Guest => "Gast".to_string(),
                };
                html! {
                    <div class="role">{role}</div>
                }
            })
            .collect();

        html! {
            <div class="user">
                <div class="label">{"Benutzer: "}</div>
                <div class="username">{user.name}</div>
                <div class="label">{"Rollen: "}</div>
                { roles }
                if let Some(url) = logout_url {
                    <a href={url}>{"Abmelden"}</a>
                }
            </div>
        }
    } else {
        html! {
            <div class="user">
                if let Some(url) = login_url {
                    <a href={url}>{"Anmelden"}</a>
                }
            </div>
        }
    }
}
