use yew::prelude::*;
use yew_router::prelude::*;
use crate::{context::auth::use_auth, Route};

use super::auth::LoginComponent;

#[function_component(Login)]
pub fn login() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    // Redirect if already logged in
    if auth.token.is_some() {
        navigator.push(&Route::Home);
    }
    html! {
        <div class="container text-center bg-color-secondary text-black p-5">
            <LoginComponent />
        </div>
    }
}
