use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
    pub set_token: Callback<Option<String>>,
    pub access_token: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let token = use_state(|| None::<String>);
    let access_token = (*token).clone();

    let set_token = {
        let token = token.clone();
        Callback::from(move |new_token: Option<String>| {
            token.set(new_token);
        })
    };

    let auth_context = AuthContext {
        token: (*token).clone(),
        set_token,
        access_token,
    };

    html! {
        <ContextProvider<AuthContext> context={auth_context}>
            { for props.children.iter() }
        </ContextProvider<AuthContext>>
    }
}

#[hook]
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not found")
}
