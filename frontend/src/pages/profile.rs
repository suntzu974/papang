use crate::services::api_service::ApiService;
use gloo_net::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::TargetCast;
use yew_router::prelude::*;
use crate::{context::auth::{use_auth, check_auth_response}, Route};

#[derive(Serialize)]
struct UpdateNamePayload {
    name: String,
}

#[derive(Serialize)]
struct ChangePasswordPayload {
    current_password: String,
    new_password: String,
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let is_editing = use_state(|| false);
    let new_name = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());
    let show_password_form = use_state(|| false);
    let current_password = use_state(|| "".to_string());
    let new_password = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());
    let password_message = use_state(|| "".to_string());
    let password_strength = use_state(|| 0);

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

                    let res = ApiService::put("/auth/me")
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("Content-Type", "application/json")
                        .json(&payload)
                        .unwrap()
                        .send()
                        .await;

                    match res {
                        Ok(resp) => {
                            if check_auth_response(resp.status(), &auth) {
                                if resp.status() == 200 {
                                    response_message.set("Nom mis à jour !".to_string());
                                    is_editing.set(false);
                                    
                                    // Refresh user data
                                    let refresh_res = ApiService::get("/auth/me")
                                        .header("Authorization", &format!("Bearer {}", token))
                                        .send()
                                        .await;
                                    
                                    if let Ok(refresh_resp) = refresh_res {
                                        if refresh_resp.status() == 200 {
                                            if let Ok(updated_user) = refresh_resp.json().await {
                                                auth.set_user.emit(Some(updated_user));
                                            }
                                        }
                                    }
                                } else {
                                    response_message.set("Erreur lors de la mise à jour".to_string());
                                }
                            }
                        }
                        Err(_) => response_message.set("Erreur réseau".to_string()),
                    }
                }
            });
        })
    };

    let on_change_password = {
        let current_password = current_password.clone();
        let new_password = new_password.clone();
        let confirm_password = confirm_password.clone();
        let password_message = password_message.clone();
        let show_password_form = show_password_form.clone();
        let auth = auth.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let current_password = current_password.clone();
            let new_password = new_password.clone();
            let confirm_password = confirm_password.clone();
            let password_message = password_message.clone();
            let show_password_form = show_password_form.clone();
            let auth = auth.clone();

            // Validation
            if (*new_password).len() < 8 {
                password_message.set("Le nouveau mot de passe doit contenir au moins 8 caractères".to_string());
                return;
            }

            if *new_password != *confirm_password {
                password_message.set("Les mots de passe ne correspondent pas".to_string());
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth.token {
                    let payload = ChangePasswordPayload {
                        current_password: (*current_password).clone(),
                        new_password: (*new_password).clone(),
                    };

                    let res = ApiService::put("/auth/change-password")
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("Content-Type", "application/json")
                        .json(&payload)
                        .unwrap()
                        .send()
                        .await;

                    match res {
                        Ok(resp) => {
                            if check_auth_response(resp.status(), &auth) {
                                if resp.status() == 200 {
                                    password_message.set("Mot de passe modifié avec succès !".to_string());
                                    current_password.set("".to_string());
                                    new_password.set("".to_string());
                                    confirm_password.set("".to_string());
                                    show_password_form.set(false);
                                } else if resp.status() == 400 {
                                    password_message.set("Mot de passe actuel incorrect".to_string());
                                } else {
                                    password_message.set("Erreur lors de la modification".to_string());
                                }
                            }
                        }
                        Err(_) => password_message.set("Erreur réseau".to_string()),
                    }
                }
            });
        })
    };

    // Password strength calculator
    let calculate_password_strength = {
        let password_strength = password_strength.clone();
        move |password: &str| {
            let mut strength = 0;
            if password.len() >= 8 { strength += 1; }
            if password.chars().any(|c| c.is_lowercase()) { strength += 1; }
            if password.chars().any(|c| c.is_uppercase()) { strength += 1; }
            if password.chars().any(|c| c.is_numeric()) { strength += 1; }
            if password.chars().any(|c| !c.is_alphanumeric()) { strength += 1; }
            password_strength.set(strength);
        }
    };

    html! {
        <div class="container-fluid">
            <div class="row justify-content-center">
                <div class="col-12 col-sm-10 col-md-8 col-lg-6 col-xl-5">
                    <div class="d-flex justify-content-between align-items-center mb-4">
                        <h2 class="mb-0">{ "Mon Profil" }</h2>
                        {
                            if *show_password_form {
                                html! {
                                    <span class="badge bg-secondary">{ "Modification du mot de passe" }</span>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    
                    {
                        if let Some(user) = &auth.user {
                            html! {
                                <>
                                    {
                                        if !*show_password_form {
                                            html! {
                                                <div class="card mb-4 shadow-sm">
                                                    <div class="card-header bg-primary text-white">
                                                        <h6 class="mb-0">{ "Informations personnelles" }</h6>
                                                    </div>
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
                                                            if let Some(verified) = user.email_verified {
                                                                html! {
                                                                    <p class="card-text">
                                                                        { "Email vérifié: " }
                                                                        {
                                                                            if verified {
                                                                                html! { <span class="badge bg-success">{ "Oui" }</span> }
                                                                            } else {
                                                                                html! { <span class="badge bg-warning">{ "Non" }</span> }
                                                                            }
                                                                        }
                                                                    </p>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }
                                                        }
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
                                            html! {}
                                        }
                                    }

                                    <div class="card shadow-sm">
                                        <div class="card-header bg-warning text-dark d-flex justify-content-between align-items-center">
                                            <h6 class="mb-0">{ "Sécurité" }</h6>
                                            <button 
                                                class="btn btn-sm btn-outline-dark"
                                                onclick={{
                                                    let show_password_form = show_password_form.clone();
                                                    Callback::from(move |_| show_password_form.set(!*show_password_form))
                                                }}
                                            >
                                                { if *show_password_form { "✕ Annuler" } else { "🔒 Changer le mot de passe" } }
                                            </button>
                                        </div>
                                        <div class="card-body">
                                            {
                                                if *show_password_form {
                                                    html! {
                                                        <div class="container-fluid p-0">
                                                            <form onsubmit={on_change_password} class="needs-validation" novalidate=true>
                                                                <div class="row">
                                                                    <div class="col-12">
                                                                        <div class="mb-3">
                                                                            <label class="form-label fw-bold">{ "Mot de passe actuel" }</label>
                                                                            <input
                                                                                type="password"
                                                                                class="form-control form-control-lg"
                                                                                placeholder="••••••••"
                                                                                value={(*current_password).clone()}
                                                                                required=true
                                                                                oninput={{
                                                                                    let current_password = current_password.clone();
                                                                                    Callback::from(move |e: InputEvent| {
                                                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                                                        current_password.set(input.value());
                                                                                    })
                                                                                }}
                                                                            />
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                                
                                                                <div class="row">
                                                                    <div class="col-12">
                                                                        <div class="mb-3">
                                                                            <label class="form-label fw-bold">{ "Nouveau mot de passe" }</label>
                                                                            <input
                                                                                type="password"
                                                                                class="form-control form-control-lg"
                                                                                placeholder="Minimum 8 caractères"
                                                                                value={(*new_password).clone()}
                                                                                required=true
                                                                                oninput={{
                                                                                    let new_password = new_password.clone();
                                                                                    let calculate_strength = calculate_password_strength.clone();
                                                                                    Callback::from(move |e: InputEvent| {
                                                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                                                        let value = input.value();
                                                                                        calculate_strength(&value);
                                                                                        new_password.set(value);
                                                                                    })
                                                                                }}
                                                                            />
                                                                            // Password strength indicator
                                                                            {
                                                                                if !(*new_password).is_empty() {
                                                                                    let strength = *password_strength;
                                                                                    let (color_class, strength_text, width) = match strength {
                                                                                        0..=1 => ("bg-danger", "Très faible", "20%"),
                                                                                        2 => ("bg-warning", "Faible", "40%"),
                                                                                        3 => ("bg-info", "Moyen", "60%"),
                                                                                        4 => ("bg-success", "Fort", "80%"),
                                                                                        _ => ("bg-success", "Très fort", "100%"),
                                                                                    };
                                                                                    html! {
                                                                                        <div class="mt-3 p-3 bg-light rounded">
                                                                                            <div class="d-flex justify-content-between align-items-center mb-2">
                                                                                                <small class="text-muted fw-semibold">{ "Force:" }</small>
                                                                                                <small class="fw-bold text-uppercase">{ strength_text }</small>
                                                                                            </div>
                                                                                            <div class="progress mb-2" style="height: 8px;">
                                                                                                <div 
                                                                                                    class={format!("progress-bar {} progress-bar-striped", color_class)} 
                                                                                                    style={format!("width: {}; transition: width 0.3s ease;", width)}
                                                                                                    role="progressbar"
                                                                                                ></div>
                                                                                            </div>
                                                                                            <small class="text-muted d-block">
                                                                                                { "✓ 8+ caractères ✓ Majuscules ✓ Minuscules ✓ Chiffres ✓ Symboles" }
                                                                                            </small>
                                                                                        </div>
                                                                                    }
                                                                                } else {
                                                                                    html! {}
                                                                                }
                                                                            }
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                                
                                                                <div class="row">
                                                                    <div class="col-12">
                                                                        <div class="mb-4">
                                                                            <label class="form-label fw-bold">{ "Confirmer le nouveau mot de passe" }</label>
                                                                            <input
                                                                                type="password"
                                                                                class={
                                                                                    if !(*confirm_password).is_empty() && *new_password != *confirm_password {
                                                                                        "form-control form-control-lg is-invalid"
                                                                                    } else if !(*confirm_password).is_empty() && *new_password == *confirm_password {
                                                                                        "form-control form-control-lg is-valid"
                                                                                    } else {
                                                                                        "form-control form-control-lg"
                                                                                    }
                                                                                }
                                                                                placeholder="Répétez le nouveau mot de passe"
                                                                                value={(*confirm_password).clone()}
                                                                                required=true
                                                                                oninput={{
                                                                                    let confirm_password = confirm_password.clone();
                                                                                    Callback::from(move |e: InputEvent| {
                                                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                                                        confirm_password.set(input.value());
                                                                                    })
                                                                                }}
                                                                            />
                                                                            {
                                                                                if !(*confirm_password).is_empty() && *new_password != *confirm_password {
                                                                                    html! {
                                                                                        <div class="invalid-feedback">
                                                                                            { "Les mots de passe ne correspondent pas" }
                                                                                        </div>
                                                                                    }
                                                                                } else if !(*confirm_password).is_empty() && *new_password == *confirm_password {
                                                                                    html! {
                                                                                        <div class="valid-feedback">
                                                                                            { "Les mots de passe correspondent" }
                                                                                        </div>
                                                                                    }
                                                                                } else {
                                                                                    html! {}
                                                                                }
                                                                            }
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                                
                                                                <div class="row">
                                                                    <div class="col-12">
                                                                        <div class="d-grid">
                                                                            <button 
                                                                                type="submit" 
                                                                                class="btn btn-primary btn-lg"
                                                                                disabled={
                                                                                    (*current_password).is_empty() || 
                                                                                    (*new_password).len() < 8 || 
                                                                                    *new_password != *confirm_password
                                                                                }
                                                                            >
                                                                                <i class="bi bi-shield-check me-2"></i>
                                                                                { "Modifier le mot de passe" }
                                                                            </button>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </form>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {
                                                        <div class="text-center py-4">
                                                            <i class="bi bi-shield-lock display-4 text-muted mb-3"></i>
                                                            <p class="text-muted mb-0">{ "Cliquez sur 'Changer le mot de passe' pour modifier votre mot de passe." }</p>
                                                        </div>
                                                    }
                                                }
                                            }
                                            {
                                                if !(*password_message).is_empty() {
                                                    let alert_class = if (*password_message).contains("succès") {
                                                        "alert alert-success mt-3"
                                                    } else {
                                                        "alert alert-danger mt-3"
                                                    };
                                                    html! {
                                                        <div class={alert_class} role="alert">
                                                            { (*password_message).clone() }
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                    </div>
                                </>
                            }
                        } else {
                            html! { 
                                <div class="text-center py-5">
                                    <div class="spinner-border text-primary" role="status">
                                        <span class="visually-hidden">{ "Chargement..." }</span>
                                    </div>
                                    <p class="mt-3 text-muted">{ "Chargement du profil..." }</p>
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}
