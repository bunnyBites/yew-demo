use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    label: AttrValue,
    name: AttrValue,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html {
    html!{
        <div class="mb-3">
            <label>{props.label.clone()}</label>
            <input type="text" />
        </div>
    }
}