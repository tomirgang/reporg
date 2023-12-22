use crate::app::AppState;
use yew::prelude::*;

#[function_component(User)]
pub fn user() -> Html {
    let state = use_context::<AppState>();
    let user = match state {
        Some(state) => state.user,
        None => None,
    };

    if let Some(user) = user {
        html! {
            <div class="user">
                <div class="user">{user.name}</div>

                <a href="/api/user/logout">{"Abmelden"}</a>
            </div>
        }
    } else {
        html! {
            <div class="user">
                <a href="">{"Anmelden"}</a>
            </div>
        }
    }
}
