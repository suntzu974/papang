use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::auth::use_auth;

// Define routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/expenses")]
    Expenses,
    #[at("/expenses/add")]
    AddExpense,
    #[at("/expenses/manage")]
    ManageExpenses,
    #[at("/profile")]
    Profile,
    #[at("/logout")]
    Logout,
}

#[function_component(Header)]
pub fn header() -> Html {
    let auth = use_auth();
    html! {
        <header>
            <div class="bg-primary text-white text-center py-2 mb-3">
                <h1 class="m-0">{ "Papang - Gestion des Dépenses" }</h1>
            </div>
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
pub fn navbar() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    console::log_1(&"Rendering Navbar".into());
    console::log_1(&format!("Access Token: {:?}", auth.access_token).into());
    console::log_1(&format!("User: {:?}", auth.user).into());
    let on_logout = {
        let set_token = auth.set_token.clone();
        let set_user = auth.set_user.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            set_token.emit(None);
            set_user.emit(None);
            navigator.push(&Route::Login);
        })
    };
    
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark">
            <div class="container-fluid justify-content-center">
                <div class="navbar-nav">
                    {
                        if auth.access_token.is_some() {
                            console::log_1(&"User is authenticated in NavBar".into());
                            html! {
                                <>
                                    <Link<Route> to={Route::Home} classes="nav-link text-black mx-2">
                                        <i class="bi bi-house"></i>{ " Accueil" }
                                    </Link<Route>>
                                    <Link<Route> to={Route::AddExpense} classes="nav-link text-black mx-2">
                                        <i class="bi bi-plus-circle"></i>{ " Ajouter" }
                                    </Link<Route>>
                                    <Link<Route> to={Route::ManageExpenses} classes="nav-link text-black mx-2">
                                        <i class="bi bi-list-ul"></i>{ " Gérer" }
                                    </Link<Route>>
                                    <Link<Route> to={Route::Profile} classes="nav-link text-black mx-2">
                                        <i class="bi bi-person"></i>{ " Profil" }
                                    </Link<Route>>
                                    <button 
                                        class="btn  btn-sm mx-2 color-secondary"
                                        onclick={on_logout}
                                    >
                                        <i class="bi bi-box-arrow-right"></i>{ " Déconnexion" }
                                    </button>
                                </>
                            }
                        } 
                        else {
                            html! {
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
