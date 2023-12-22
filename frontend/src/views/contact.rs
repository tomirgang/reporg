use yew::prelude::*;

use crate::components::title::{single_title, Title};

#[function_component(Contact)]
pub fn contact() -> Html {
    gloo_utils::document().set_title("RepOrg - Kontakt");

    let title = single_title("Kontakt", Some("/img/contact.jpg"));

    html! {
        <>
            <Title ..title />
        </>
    }
}
