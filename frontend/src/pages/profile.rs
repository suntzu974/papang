use yew::prelude::*;
use yew_router::prelude::*;
use crate::{context::auth::use_auth, Route};

#[function_component(Profile)]
pub fn profile() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    // Redirect if not logged in
    if auth.token.is_none() {
        navigator.push(&Route::Login);
    }

    html! {
        <div class="container">
            <h2>{ "Mon Profil" }</h2>
            {
                if let Some(user) = &auth.user {
                    html! {
                        <div class="card">
                            <div class="card-body">
                                <h5 class="card-title">{ &user.name }</h5>
                                <p class="card-text">{ format!("Email: {}", &user.email) }</p>
                                <p class="card-text">{ format!("ID: {}", user.id) }</p>
                            </div>
                        </div>
                    }
                } else {
                    html! { <p>{ "Chargement..." }</p> }
                }
            }
        </div>
    }
}
