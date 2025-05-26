use yew::prelude::*;
use yew_router::prelude::*;
use crate::{context::auth::use_auth, Route};

#[function_component(Login)]
pub fn login() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    // Redirect if already logged in
    if auth.token.is_some() {
        navigator.push(&Route::Home);
    }

    html! {
        <div class="container">
            <h2>{ "Connexion" }</h2>
            // Add your login form here
        </div>
    }
}
