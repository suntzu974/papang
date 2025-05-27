use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component(Layout)]
fn layout() -> Html {
    html! {
        <div class="d-flex flex-column min-vh-100">
            <header>
                <BrowserRouter>
                    <nav class="navbar navbar-expand-lg navbar-light bg-primary">
                        <div class="container">
                            <a class="navbar-brand text-white" href="#">{ "Mon Application" }</a>
                            <div class="collapse navbar-collapse">
                                <ul class="navbar-nav ms-auto">
                                    <li class="nav-item">
                                        <Link<Route> classes="nav-link text-white" to={Route::Home}>{ "Accueil" }</Link<Route>>
                                    </li>
                                    <li class="nav-item">
                                        <Link<Route> classes="nav-link text-white" to={Route::About}>{ "à propos" }</Link<Route>>
                                    </li>
                                    <li class="nav-item">
                                        <Link<Route> classes="nav-link text-white" to={Route::Contact}>{ "Contact" }</Link<Route>>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </nav>
                </BrowserRouter>
            </header>
            <main class="flex-fill container py-4">
                <Switch<Route> render={switch} />
            </main>
            <footer class="bg-dark text-white-50 text-center py-3 mt-auto">
                { "© 2023 Mon Application" }
            </footer>
        </div>
    }
}
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::Contact => html! { <ContactPage /> },
        Route::NotFound => html! { <h1>{"404 - Page non trouv�e"}</h1> },
    }
}

#[function_component(HomePage)]
fn home_page() -> Html {
    html! { <h2>{"Page d'accueil"}</h2> }
}

#[function_component(AboutPage)]
fn about_page() -> Html {
    html! { <h2>{"� propos"}</h2> }
}

#[function_component(ContactPage)]
fn contact_page() -> Html {
    html! { <h2>{"Contactez-nous"}</h2> }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <Layout />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}