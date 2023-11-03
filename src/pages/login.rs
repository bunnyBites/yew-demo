use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{self};

#[function_component]
pub fn Login() -> Html {
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();

    let update_username = Callback::from(move |e: Event| {
        e.prevent_default();
        let input_element = e.target_dyn_into::<HtmlInputElement>();

        if let Some(input) = input_element {
            username_handle.set(input.value());
        }
    });

    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();

    let update_password = Callback::from(move |e: Event| {
        let input_element = e.target_dyn_into::<HtmlInputElement>();

        if let Some(input) = input_element {
            password_handle.set(input.value());
        }
    });

    let cloned_username = username.clone();
    let cloned_password = password.clone();

    let submit_form = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        log!(
            "user details",
            cloned_username.clone(),
            cloned_password.clone()
        );
    });

    html! {
        <form onsubmit={submit_form}>
            <components::input::Input
                label="Username"
                field_type="text"
                name="username"
                onchange={update_username}
                value={username}
            />
            <components::input::Input
                label="Password"
                field_type="password"
                name="password"
                onchange={update_password}
                value={password}
            />
            <button type="submit" class="btn btn-info w-100">{"Login"}</button>
        </form>
    }
}
