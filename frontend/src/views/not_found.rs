use yew::prelude::*;

use crate::components::title::{single_title, Title};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    gloo_utils::document().set_title("RepOrg - 404");

    let title = single_title("Seite nicht gefunden", Some("/img/404.jpg"));

    html! {
        <>
            <Title ..title />
        </>
    }
}
