use yew::prelude::*;
use crate::context::auth::use_auth;
use crate::pages::auth::LoginComponent;
use crate::pages::dashboard::ExpenseDashboard;

#[function_component(Home)]
pub fn home_component() -> Html {
    let auth = use_auth();

    html! {
        if auth.access_token.is_some() {
            <ExpenseDashboard />
        } else {
            <LoginComponent /> 
        }
    }
}
