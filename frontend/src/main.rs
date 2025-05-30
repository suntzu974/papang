use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod context;
mod types;
mod services;
use components::layout::{Header, Footer};
use pages::{
    main::Main,
    login::Login,
    register::Register,

    profile::Profile,
};
use context::auth::AuthProvider;
use crate::components::layout::Navbar;
use crate::pages::{
    manage_expenses::ManageExpenses,
    add_expense::AddExpense,
};

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
    #[at("/expenses/add")]
    AddExpense,
    #[at("/expenses/manage")]
    ManageExpenses,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Main /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },

        Route::Profile => html! { <Profile /> },
        Route::Logout => html! { <LogoutPage /> },
        Route::AddExpense => html! { <AddExpense /> },
        Route::ManageExpenses => html! { <ManageExpenses /> }, 
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
                    <Switch<Route> render={switch} />
            </BrowserRouter>
        </AuthProvider>
    }
}

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}