use crate::components::title::{dual_title, Title};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    gloo_utils::document().set_title("RepOrg");

    let title = dual_title(
        "Wegwerfen? Denkste!",
        Some("img/kaputt.jpg"),
        "Komm zum Repair-Café!",
        Some("img/reparieren.jpg"),
    );

    html! {
        <>
            <Title ..title />
        </>
    }
}
