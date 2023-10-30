use yew::prelude::*;

mod pages;
mod components;

#[function_component]
// the name of this function is passed to the renderer struct
fn AppRoot() -> Html {
    html!(
        <div class="card w-50 mx-auto">
            <div class="card-body">
                <pages::login::Login />
            </div>
        </div>
    )
}

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}
