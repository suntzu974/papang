use yew::prelude::*;
use crate::context::auth::use_auth;
use crate::pages::auth::LoginComponent;
use crate::pages::expenses::ExpenseComponent;

#[function_component(Main)]
pub fn main_component() -> Html {
    let auth = use_auth();

    html! {
        if auth.access_token.is_some() {
            <ExpenseComponent /> 
        } else {
            <LoginComponent /> 
        }
    }
}
