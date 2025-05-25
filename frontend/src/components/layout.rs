use yew::prelude::*;
use crate::context::auth::use_auth;

#[function_component(Header)]
pub fn header() -> Html {
    let auth = use_auth();
    html! {
        <header class="bg-dark text-white py-3">
            <div class="container">
                <h1 class="text-center mb-3">{ "Papang - Gestion des Dépenses" }</h1>
                <Navbar />
                {
                    if let Some(token) = &auth.token {
                        html! {
                            <div class="text-center mt-3">
                                <small class="text-success">
                                    { format!("Connecté - Token: {}...", &token[..token.len().min(20)]) }
                                </small>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </header>
    }
}

#[function_component(Navbar)]
fn navbar() -> Html {
    let auth = use_auth();
    
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark">
            <div class="container-fluid justify-content-center">
                <div class="navbar-nav">
                    {
                        if auth.access_token.is_some() {
                            html! {
                                <>
                                    <a class="nav-link text-white mx-2" href="#">
                                        <i class="bi bi-list-ul"></i>{ " Dépenses" }
                                    </a>
                                    <button 
                                        class="btn btn-outline-light btn-sm mx-2"
                                        onclick={{
                                            let set_token = auth.set_token.clone();
                                            Callback::from(move |_| set_token.emit(None))
                                        }}
                                    >
                                        { "Déconnexion" }
                                    </button>
                                </>
                            }
                        } else {
                            html! {
                                <span class="nav-link text-muted">{ "Non connecté" }</span>
                            }
                        }
                    }
                </div>
            </div>
        </nav>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-dark text-white text-center py-3 mt-auto">
            <div class="container">
                <span>{ "© 2024 Papang - Gestion des Dépenses" }</span>
            </div>
        </footer>
    }
}
