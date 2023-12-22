use yew::prelude::*;

use crate::components::side_menu::Menu;
use crate::components::title::{single_title, Title};

#[function_component(One)]
pub fn one() -> Html {
    gloo_utils::document().set_title("RepOrg - Mitarbeiter - One");

    let title = single_title("Mitarbeiter - One", None);

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
