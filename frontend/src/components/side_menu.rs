use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::MembersRoute;
use crate::components::user::User;

struct SideNavItem {
    link: MembersRoute,
    label: String,
}

#[function_component(Menu)]
pub fn menu() -> Html {
    let nav_items: UseStateHandle<Vec<SideNavItem>> = use_state(|| {
        vec![
            SideNavItem {
                link: MembersRoute::One,
                label: "One".to_owned(),
            },
            SideNavItem {
                link: MembersRoute::Two,
                label: "Two".to_owned(),
            },
            SideNavItem {
                link: MembersRoute::Three,
                label: "Three".to_owned(),
            },
        ]
    });

    let route = use_route::<MembersRoute>().unwrap();

    html! {
        <div id="side_menu" class="side_menu">
            <User />
            {
                nav_items.iter().map(|nav_item| {
                    let is_active = nav_item.link == route;
                    html! {
                        <Link<MembersRoute>
                            to={nav_item.link.clone()}
                            classes={classes!(if is_active {"active"} else { "" })}
                        >
                            {nav_item.label.clone()}
                        </Link<MembersRoute>>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
