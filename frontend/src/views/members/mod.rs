pub mod one;
pub mod three;
pub mod two;

use yew::prelude::*;

use crate::components::side_menu::Menu;
use crate::components::title::{single_title, Title};

#[function_component(Members)]
pub fn members() -> Html {
    gloo_utils::document().set_title("RepOrg - Mitarbeiter");

    let title = single_title("Mitarbeiter", None);

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
