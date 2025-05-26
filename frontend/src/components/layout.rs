use yew::prelude::*;
use crate::context::auth::use_auth;

#[function_component(Header)]
pub fn header() -> Html {
    let auth = use_auth();
    html! {
        <header>
            <nav class="navbar navbar-dark bg-dark py-3 mb-4">
                <div class="container">
                    <span class="navbar-brand mx-auto h1 mb-0">{ "Papang - Gestion des Dépenses" }</span>
                </div>
            </nav>
            <Navbar />
            {
                if let Some(user) = &auth.user {
                    html! {
                        <div class="container">
                            <div class="alert alert-success text-center py-2 mb-0" role="alert">
                                <small>
                                    { format!("Connecté - {}", user.name) }
                                </small>
                            </div>
                        </div>
                    }
                } else if auth.token.is_some() {
                    html! {
                        <div class="container">
                            <div class="alert alert-info text-center py-2 mb-0" role="alert">
                                <small>{ "Connecté" }</small>
                            </div>
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
        <footer class="footer py-3 bg-dark fixed-bottom">
            <div class="container text-center">
                <span class="text-white-50">{ "© 2024 Papang - Gestion des Dépenses" }</span>
            </div>
        </footer>
    }
}
