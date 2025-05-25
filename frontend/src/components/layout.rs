use yew::prelude::*;
use crate::context::auth::use_auth;

#[function_component(Header)]
pub fn header() -> Html {
    let auth = use_auth();
    html! {
        <header style="padding: 1em; background: #222; color: #fff; text-align: center;">
            <h1>{ "Papang - Gestion des Dépenses" }</h1>
            <Navbar />
            {
                if let Some(token) = &auth.token {
                    html! {
                        <div style="margin-top: 1em; font-size: 0.9em; color: #b2ffb2;">
                            { format!("Access token: {}", token) }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </header>
    }
}

#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
        <nav style="margin-top: 1em;">
            <a href="#" style="color: #fff; margin: 0 1em; text-decoration: underline;">{ "Dépenses" }</a>
        </nav>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer style="padding: 1em; background: #222; color: #fff; text-align: center; position: fixed; width: 100%; bottom: 0;">
            <span>{ "© 2024 Papang" }</span>
        </footer>
    }
}
