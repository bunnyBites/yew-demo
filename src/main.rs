use yew::prelude::*;

#[function_component]
// the name of this function is passed to the renderer struct
fn AppRoot() -> Html {
    html!(
        <div>{"hello world!"}</div>
    )
}

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}
