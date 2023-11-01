use crate::components::login_form;
use yew::prelude::*;

mod components;
mod pages;

#[function_component]
// the name of this function is passed to the renderer struct
fn AppRoot() -> Html {
    html!(
        <>
            // appbar section
            <components::appbar::Appbar />

            // Login section
            <login_form::LoginForm />
        </>
    )
}

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}
