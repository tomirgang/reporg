use crate::components::nav::Nav;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{
    contact::Contact, home::Home, members::one::One, members::three::Three, members::two::Two,
    members::Members, not_found::NotFound, register::Register,
};

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
    pub role: Role,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub user: Option<UserData>,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState { user: None });

    html! {
        <ContextProvider<AppState> context={(*state).clone()}>
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </ContextProvider<AppState>>
    }
}
