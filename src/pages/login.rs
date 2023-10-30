use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
        <form>
            <div>
                <label>{"Username:"}</label>
                <input type="text" />
            </div>
        </form>
    }
}