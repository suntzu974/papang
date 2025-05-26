use yew::prelude::*;
use yew_router::prelude::*;
use crate::{context::auth::use_auth, Route};

#[function_component(Register)]
pub fn register() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    // Redirect if already logged in
    if auth.token.is_some() {
        navigator.push(&Route::Home);
    }

    html! {
        <div class="container">
            <h2>{ "Inscription" }</h2>
            // Add your register form here
        </div>
    }
}
