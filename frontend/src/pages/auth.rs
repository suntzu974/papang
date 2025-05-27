use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use crate::context::auth::use_auth;
use crate::types::{LoginForm, LoginResponse, RegisterForm, RegisterResponse};

#[function_component(LoginComponent)]
pub fn login_component() -> Html {
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());
    let show_register = use_state(|| false);
    let auth_context = use_auth();

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let response_message = response_message.clone();
        let show_register = show_register.clone();
        let set_token = auth_context.set_token.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = email.clone();
            let password = password.clone();
            let response_message = response_message.clone();
            let show_register = show_register.clone();
            let set_token = set_token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let login_data = LoginForm {
                    email: (*email).clone(),
                    password: (*password).clone(),
                };

                let res = Request::post("http://localhost:3001/auth/login")
                    .header("Content-Type", "application/json")
                    .json(&login_data)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            let json = resp.json::<LoginResponse>().await.unwrap();
                            let access_token = json.access_token.clone();
                            set_token.emit(Some(access_token));
                            response_message.set("Login réussi".to_string());
                        } else if resp.status() == 401 {
                            show_register.set(true);
                            response_message.set("Identifiants invalides - Redirection vers l'inscription".to_string());
                        } else {
                            response_message.set("Erreur serveur".to_string());
                        }
                    }
                    Err(_) => {
                        response_message.set("Erreur de connexion".to_string());
                    }
                }
            });
        })
    };

    if *show_register {
        html! {
            <div class="container mt-5">
                <div class="row justify-content-center">
                    <div class="col-md-6">
                        <RegisterComponent />
                        <div class="text-center mt-3">
                            <button 
                                class="btn btn-secondary"
                                onclick={{
                                    let show_register = show_register.clone();
                                    Callback::from(move |_| show_register.set(false))
                                }}
                            >{ "Retour à la connexion" }</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="container mt-5">
                <div class="row justify-content-center">
                    <div class="col-md-6">
                        <div class="card">
                            <div class="card-body">
                                <h1 class="card-title text-center mb-4">{ "Connexion" }</h1>
                                <form onsubmit={on_submit}>
                                    <div class="mb-3">
                                        <input
                                            type="email"
                                            class="form-control"
                                            placeholder="Email"
                                            value={(*email).clone()}
                                            oninput={Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                email.set(input.value());
                                            })}
                                        />
                                    </div>
                                    <div class="mb-3">
                                        <input
                                            type="password"
                                            class="form-control"
                                            placeholder="Mot de passe"
                                            value={(*password).clone()}
                                            oninput={Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                password.set(input.value());
                                            })}
                                        />
                                    </div>
                                    <div class="d-grid mb-3">
                                        <button type="submit" class="btn btn-primary">{ "Se connecter" }</button>
                                    </div>
                                </form>
                                <div class="d-grid">
                                    <button 
                                        class="btn btn-outline-secondary"
                                        onclick={{
                                            let show_register = show_register.clone();
                                            Callback::from(move |_| show_register.set(true))
                                        }}
                                    >{ "S'inscrire" }</button>
                                </div>
                                {
                                    if !(*response_message).is_empty() {
                                        html! {
                                            <div class="alert alert-info mt-3" role="alert">
                                                { (*response_message).clone() }
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

#[function_component(RegisterComponent)]
pub fn register_component() -> Html {
    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());
    let access_token = use_state(|| None::<String>);
    let auth_context = use_auth();

    let on_submit = {
        let name = name.clone();
        let email = email.clone();
        let password = password.clone();
        let response_message = response_message.clone();
        let access_token = access_token.clone();
        let set_token = auth_context.set_token.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let email = email.clone();
            let password = password.clone();
            let response_message = response_message.clone();
            let access_token = access_token.clone();
            let set_token = set_token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let register_data = RegisterForm {
                    name: (*name).clone(),
                    email: (*email).clone(),
                    password: (*password).clone(),
                };

                let res = Request::post("http://localhost:3001/auth/register")
                    .header("Content-Type", "application/json")
                    .json(&register_data)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) => {
                        if resp.status() == 201 {
                            match resp.json::<RegisterResponse>().await {
                                Ok(json) => {
                                    access_token.set(Some(json.access_token.clone()));
                                    set_token.emit(Some(json.access_token.clone()));
                                    response_message.set("Inscription réussie".to_string());
                                }
                                Err(_) => {
                                    response_message.set("Erreur lors de la récupération du token".to_string());
                                }
                            }
                        } else if resp.status() == 409 {
                            response_message.set("Utilisateur déjà existant".to_string());
                        } else {
                            response_message.set("Erreur serveur".to_string());
                        }
                    }
                    Err(_) => {
                        response_message.set("Erreur de connexion".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="card">
            <div class="card-body">
                <h1 class="card-title text-center mb-4">{ "Inscription" }</h1>
                <form onsubmit={on_submit}>
                    <div class="mb-3">
                        <input
                            type="text"
                            class="form-control"
                            placeholder="Nom"
                            value={(*name).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                name.set(input.value());
                            })}
                        />
                    </div>
                    <div class="mb-3">
                        <input
                            type="email"
                            class="form-control"
                            placeholder="Email"
                            value={(*email).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                email.set(input.value());
                            })}
                        />
                    </div>
                    <div class="mb-3">
                        <input
                            type="password"
                            class="form-control"
                            placeholder="Mot de passe"
                            value={(*password).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                password.set(input.value());
                            })}
                        />
                    </div>
                    <div class="d-grid">
                        <button type="submit" class="btn btn-success">{ "S'inscrire" }</button>
                    </div>
                </form>
                {
                    if !(*response_message).is_empty() {
                        html! {
                            <div class="alert alert-info mt-3" role="alert">
                                { (*response_message).clone() }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(token) = &auth_context.access_token {
                        html! { 
                            <div class="alert alert-success mt-3" role="alert">
                                <small>{ format!("Access token: {}", token) }</small>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
