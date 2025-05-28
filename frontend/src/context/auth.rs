use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
    pub set_token: Callback<Option<String>>,
    pub access_token: Option<String>,
    pub user: Option<User>,
    pub set_user: Callback<Option<User>>,
    pub logout: Callback<()>,
    pub handle_session_expired: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let token = use_state(|| None::<String>);
    let user = use_state(|| None::<User>);
    let access_token = (*token).clone();

    let set_token = {
        let token = token.clone();
        Callback::from(move |new_token: Option<String>| {
            token.set(new_token);
        })
    };

    let set_user = {
        let user = user.clone();
        Callback::from(move |new_user: Option<User>| {
            user.set(new_user);
        })
    };

    let logout = {
        let token = token.clone();
        let user = user.clone();
        Callback::from(move |_| {
            token.set(None);
            user.set(None);
        })
    };

    let handle_session_expired = {
        let logout = logout.clone();
        Callback::from(move |_| {
            logout.emit(());
            web_sys::console::log_1(&"Session expired - logged out".into());
        })
    };

    // Fetch user when token changes
    {
        let user = user.clone();
        let token = (*token).clone();
        let handle_session_expired = handle_session_expired.clone();
        use_effect_with(token, move |token| {
            if let Some(access_token) = token {
                let user = user.clone();
                let access_token = access_token.clone();
                let handle_session_expired = handle_session_expired.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let res = Request::get("http://localhost:3001/auth/me")
                        .header("Authorization", &format!("Bearer {}", access_token))
                        .send()
                        .await;
                    
                    if let Ok(resp) = res {
                        if resp.status() == 200 {
                            if let Ok(user_data) = resp.json::<User>().await {
                                user.set(Some(user_data));
                            }
                        } else if resp.status() == 401 {
                            // Token expired or invalid
                            handle_session_expired.emit(());
                        }
                    }
                });
            } else {
                user.set(None);
            }
            || ()
        });
    }

    let auth_context = AuthContext {
        token: (*token).clone(),
        set_token,
        access_token,
        user: (*user).clone(),
        set_user,
        logout,
        handle_session_expired,
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

// Add utility function for handling API responses
pub fn check_auth_response(status: u16, auth: &AuthContext) -> bool {
    if status == 401 {
        auth.handle_session_expired.emit(());
        false
    } else {
        true
    }
}
