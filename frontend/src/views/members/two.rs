use yew::prelude::*;

use crate::components::side_menu::Menu;
use crate::components::title::{single_title, Title};

#[function_component(Two)]
pub fn two() -> Html {
    gloo_utils::document().set_title("RepOrg - Mitarbeiter - Two");

    let title = single_title("Mitarbeiter - Two", None);

    html! {
        <>
            <Title ..title />
            <div class="main">
                <Menu />
                <div class="content">
                </div>
            </div>
        </>
    }
}
