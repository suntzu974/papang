use yew::prelude::*;
use yew_router::prelude::*;
use crate::{context::auth::use_auth, Route};
use crate::pages::auth::RegisterComponent;

#[function_component(Register)]
pub fn register() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    // Redirect if already logged in
    if auth.token.is_some() {
        navigator.push(&Route::Home);
    }

    html! {
        <div class="container text-center bg-color-secondary text-black p-5">
            <RegisterComponent />
        </div>
    }
}
