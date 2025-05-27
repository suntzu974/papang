use gloo_net::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::TargetCast;
use yew_router::prelude::*;
use crate::{context::auth::{use_auth, User}, Route};

#[derive(Serialize)]
struct UpdateNamePayload {
    name: String,
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let is_editing = use_state(|| false);
    let new_name = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());

    // Redirect if not logged in
    if auth.token.is_none() {
        navigator.push(&Route::Login);
    }

    let on_edit_click = {
        let is_editing = is_editing.clone();
        let new_name = new_name.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            if let Some(user) = &auth.user {
                new_name.set(user.name.clone());
            }
            is_editing.set(true);
        })
    };

    let on_cancel = {
        let is_editing = is_editing.clone();
        let response_message = response_message.clone();
        Callback::from(move |_| {
            is_editing.set(false);
            response_message.set("".to_string());
        })
    };

    let on_save = {
        let new_name = new_name.clone();
        let is_editing = is_editing.clone();
        let response_message = response_message.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            let new_name = new_name.clone();
            let is_editing = is_editing.clone();
            let response_message = response_message.clone();
            let auth = auth.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth.token {
                    let payload = UpdateNamePayload {
                        name: (*new_name).clone(),
                    };

                    let res = Request::put("http://localhost:3001/auth/me")
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("Content-Type", "application/json")
                        .json(&payload)
                        .unwrap()
                        .send()
                        .await;

                    match res {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                if let Ok(updated_user) = resp.json::<User>().await {
                                    auth.set_user.emit(Some(updated_user));
                                    response_message.set("Nom mis à jour avec succès".to_string());
                                    is_editing.set(false);
                                }
                            } else {
                                response_message.set("Erreur lors de la mise à jour".to_string());
                            }
                        }
                        Err(_) => {
                            response_message.set("Erreur de connexion".to_string());
                        }
                    }
                }
            });
        })
    };

    html! {
        <div class="container">
            <h2>{ "Mon Profil" }</h2>
            {
                if let Some(user) = &auth.user {
                    html! {
                        <div class="card">
                            <div class="card-body">
                                {
                                    if *is_editing {
                                        html! {
                                            <div class="mb-3">
                                                <label class="form-label">{ "Nom" }</label>
                                                <input
                                                    type="text"
                                                    class="form-control"
                                                    value={(*new_name).clone()}
                                                    oninput={{
                                                        let new_name = new_name.clone();
                                                        Callback::from(move |e: InputEvent| {
                                                            let input: HtmlInputElement = e.target_unchecked_into();
                                                            new_name.set(input.value());
                                                        })
                                                    }}
                                                />
                                                <div class="mt-2">
                                                    <button class="btn btn-primary me-2" onclick={on_save}>
                                                        { "Sauvegarder" }
                                                    </button>
                                                    <button class="btn btn-secondary" onclick={on_cancel}>
                                                        { "Annuler" }
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <div class="d-flex justify-content-between align-items-center">
                                                <h5 class="card-title mb-0">{ &user.name }</h5>
                                                <button class="btn btn-outline-primary btn-sm" onclick={on_edit_click}>
                                                    <i class="bi bi-pencil"></i>{ " Modifier" }
                                                </button>
                                            </div>
                                        }
                                    }
                                }
                                <p class="card-text">{ format!("Email: {}", &user.email) }</p>
                                <p class="card-text">{ format!("ID: {}", user.id) }</p>
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
                    }
                } else {
                    html! { <p>{ "Chargement..." }</p> }
                }
            }
        </div>
    }
}
