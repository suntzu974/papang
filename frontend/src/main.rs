use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod context;
mod types;

use components::layout::{Header, Footer};
use pages::{
    main::Main,
    login::Login,
    register::Register,

    profile::Profile,
};
use context::auth::AuthProvider;
use crate::components::layout::Navbar;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/profile")]
    Profile,
    #[at("/logout")]
    Logout,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Main /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },

        Route::Profile => html! { <Profile /> },
        Route::Logout => html! { <LogoutPage /> },
    }
}

#[function_component(LogoutPage)]
fn logout_page() -> Html {
    let auth = context::auth::use_auth();
    let navigator = use_navigator().unwrap();

    use_effect_with((), move |_| {
        auth.set_token.emit(None);
        auth.set_user.emit(None);
        navigator.push(&Route::Login);
        || ()
    });

    html! {
        <div class="container text-center bg-color-secondary text-white p-5">
            <p>{ "Déconnexion en cours..." }</p>
        </div>
    }
}

#[function_component(AppRoot)]
fn app_root() -> Html {
    html! {
        <AuthProvider>
            <BrowserRouter>
                <Header />
                <Navbar />
                <main style="padding-bottom: 4em;">
                    <Switch<Route> render={switch} />
                </main>
                <Footer />
            </BrowserRouter>
        </AuthProvider>
    }
}

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}