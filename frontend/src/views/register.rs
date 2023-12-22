use crate::components::input::{field_props, FieldType, InputField};
use crate::components::title::{single_title, Title};
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct RegistrationForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[function_component(Register)]
pub fn register() -> Html {
    let password_error: UseStateHandle<Option<String>> = use_state(|| None);

    let (username_ref, username_props) = field_props(
        use_node_ref(),
        FieldType::Text,
        "Username",
        "username",
        true,
    );

    let (email_ref, email_props) =
        field_props(use_node_ref(), FieldType::EMail, "E-Mail", "email", false);

    let (password_ref, password_props) = field_props(
        use_node_ref(),
        FieldType::Password,
        "Password",
        "password",
        true,
    );

    let (confirm_password_ref, mut confirm_password_props) = field_props(
        use_node_ref(),
        FieldType::Password,
        "Confirm Password",
        "confirm_password",
        true,
    );
    confirm_password_props.data.error = (*password_error).clone();

    let onsubmit = {
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let confirm_password_ref = confirm_password_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();
            let confirm_password = confirm_password_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            if password != confirm_password {
                password_error.set(Some("Passwords do not match".to_string()));
                return;
            } else {
                password_error.set(None);
            };

            let registration_form = RegistrationForm {
                username,
                email,
                password,
                confirm_password,
            };

            log::info!("registration_form {:?}", &registration_form);

            wasm_bindgen_futures::spawn_local(async move {
                let post_request = Request::post("https://reqres.in/api/register")
                    .headers({
                        let headers = Headers::new();
                        headers.set("Content-Type", "application/json");
                        headers
                    })
                    .body(JsValue::from(
                        serde_json::to_string(&registration_form).unwrap(),
                    ))
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                log::info!("post_request {:?}", &post_request);
            });
        })
    };

    gloo_utils::document().set_title("RepOrg");

    let title = single_title("Anmeldung", Some("img/anmeldung.jpg"));

    html! {
        <>
            <Title ..title />

            <form {onsubmit} class="registration-form">
                <InputField ..username_props />
                <InputField ..email_props />
                <InputField ..password_props />
                <InputField ..confirm_password_props />
                <button type="submit" class="button button-primary">{"Submit"}</button>
            </form>
        </>
    }
}
