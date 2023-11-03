use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub label: AttrValue,
    pub name: AttrValue,
    pub field_type: AttrValue,
    pub onchange: Callback<Event>,
    pub value: String,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let field_id = format!("field-{}", props.name);

    html! {
        <div class="mb-3">
            <label for={field_id.clone()} class="form-label">{props.label.clone()}</label>
            <input
                id={field_id}
                type={props.field_type.clone()}
                class="form-control"
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </div>
    }
}
