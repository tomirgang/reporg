use yew::prelude::*;

use crate::components::side_menu::Menu;
use crate::components::title::{single_title, Title};

#[function_component(Three)]
pub fn three() -> Html {
    gloo_utils::document().set_title("RepOrg - Mitarbeiter - Three");

    let title = single_title("Mitarbeiter - Three", None);

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
