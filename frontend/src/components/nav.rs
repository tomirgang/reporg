use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

struct NavItem {
    link: Route,
    label: String,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_items = use_state(|| {
        vec![
            NavItem {
                link: Route::Register,
                label: "Geräteanmeldung".to_owned(),
            },
            NavItem {
                link: Route::Contact,
                label: "Kontakt".to_owned(),
            },
            NavItem {
                link: Route::MembersRoot,
                label: "Mitarbeiter".to_owned(),
            },
        ]
    });

    let route = use_route::<Route>().unwrap();

    html! {
        <div id="topnav" class="topnav">
            <Link<Route> classes={classes!("logo")} to={Route::Home}>
                <img class={classes!{"logo_img"}} src="/img/logo.png" alt="logo" />
            </Link<Route>>
            {
                nav_items.iter().map(|nav_item| {
                    let is_active = nav_item.link == route;
                    html! {
                        <Link<Route>
                            to={nav_item.link.clone()}
                            classes={classes!(if is_active {"active"} else { "" })}
                        >
                            {nav_item.label.clone()}
                        </Link<Route>>
                    }
                }).collect::<Html>()
            }
            <a href={"javascript:toggle_menu();"} class="icon">
                <div id="menu_button" class="menu_icon_container">
                    <div class="bar1"></div>
                    <div class="bar2"></div>
                    <div class="bar3"></div>
                </div>
            </a>
        </div>
    }
}
