use crate::pages::login;
use yew::prelude::*;

#[function_component]
pub fn LoginForm() -> Html {
    html! {
        <div class="container-fluid">
            <div class="row min-vh-100 align-items-center justify-content-center">
                <div class="col-sm-4">
                    <div class="card">
                        <div class="card-body">
                            <login::Login />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
