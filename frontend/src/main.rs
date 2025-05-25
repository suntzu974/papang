use yew::prelude::*;

mod components;
mod pages;
mod context;
mod types;

use components::layout::{Header, Footer};
use pages::main::Main;
use context::auth::AuthProvider;

#[function_component(AppRoot)]
fn app_root() -> Html {
    html! {
        <AuthProvider>
            <Header />
            <main style="padding-bottom: 4em;">
                <Main />
            </main>
            <Footer />
        </AuthProvider>
    }
}

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}