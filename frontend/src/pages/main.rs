use yew::prelude::*;
use crate::context::auth::use_auth;
use crate::pages::auth::LoginComponent;
use crate::pages::dashboard::ExpenseDashboard;

#[function_component(Main)]
pub fn main_component() -> Html {
    let auth = use_auth();

    html! {
        if auth.access_token.is_some() {
            <ExpenseDashboard /> 
        } else {
            <LoginComponent /> 
        }
    }
}

// Ce composant sert de routeur conditionnel principal :
// 1. Il utilise le hook use_auth() pour accéder au contexte d'authentification
// 2. Il vérifie si l'utilisateur possède un access_token valide
// 3. Si authentifié : affiche le tableau de bord des dépenses (ExpenseDashboard)
// 4. Si non authentifié : affiche le composant de connexion (LoginComponent)

// Cette logique permet de :
// - Protéger l'accès aux fonctionnalités principales
// - Rediriger automatiquement vers la page de connexion
// - Fournir une expérience utilisateur fluide sans navigation manuelle
