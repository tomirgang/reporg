use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::MembersRoute;
use crate::app::{AppState, Role};
use crate::components::user::User;

struct SideNavItem {
    link: MembersRoute,
    label: String,
    role: Role,
}

#[function_component(Menu)]
pub fn menu() -> Html {
    let nav_items: UseStateHandle<Vec<SideNavItem>> = use_state(|| {
        vec![
            SideNavItem {
                link: MembersRoute::One,
                label: "Guest".to_owned(),
                role: Role::Guest,
            },
            SideNavItem {
                link: MembersRoute::Two,
                label: "Organizer".to_owned(),
                role: Role::Organizer,
            },
            SideNavItem {
                link: MembersRoute::Three,
                label: "Supporter".to_owned(),
                role: Role::Supporter,
            },
        ]
    });

    let route = use_route::<MembersRoute>().unwrap();

    let roles = match use_context::<AppState>() {
        Some(state) => match state.user {
            Some(user) => user.roles,
            None => vec![],
        },
        None => vec![],
    };

    html! {
        <div id="side_menu" class="side_menu">
            <User />
            {
                nav_items.iter().map(|nav_item| {
                    if roles.contains(&nav_item.role) {
                        let is_active = nav_item.link == route;
                        html! {
                            <Link<MembersRoute>
                                to={nav_item.link.clone()}
                                classes={classes!(if is_active {"active"} else { "" })}
                            >
                                {nav_item.label.clone()}
                            </Link<MembersRoute>>
                        }
                    } else {
                        html! {}
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
