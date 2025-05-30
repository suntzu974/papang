use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use serde::Serialize;

#[derive(Serialize)]
struct ResetPasswordForm {
    token: String,
    new_password: String,
}

#[derive(Properties, PartialEq)]
pub struct ResetPasswordProps {
    pub token: String,
}

#[function_component(ResetPasswordComponent)]
pub fn reset_password_component(props: &ResetPasswordProps) -> Html {
    let new_password = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());
    let is_loading = use_state(|| false);
    let password_reset_success = use_state(|| false);

    let on_submit = {
        let new_password = new_password.clone();
        let confirm_password = confirm_password.clone();
        let response_message = response_message.clone();
        let is_loading = is_loading.clone();
        let password_reset_success = password_reset_success.clone();
        let token = props.token.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let new_password = new_password.clone();
            let confirm_password = confirm_password.clone();
            let response_message = response_message.clone();
            let is_loading = is_loading.clone();
            let password_reset_success = password_reset_success.clone();
            let token = token.clone();

            // Validation
            if (*new_password).len() < 8 {
                response_message.set("Le mot de passe doit contenir au moins 8 caractères".to_string());
                return;
            }

            if *new_password != *confirm_password {
                response_message.set("Les mots de passe ne correspondent pas".to_string());
                return;
            }

            is_loading.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                let reset_data = ResetPasswordForm {
                    token,
                    new_password: (*new_password).clone(),
                };

                let res = Request::post("http://localhost:3001/auth/reset-password")
                    .header("Content-Type", "application/json")
                    .json(&reset_data)
                    .unwrap()
                    .send()
                    .await;

                is_loading.set(false);

                match res {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            password_reset_success.set(true);
                            response_message.set("Votre mot de passe a été réinitialisé avec succès. Vous pouvez maintenant vous connecter.".to_string());
                        } else {
                            response_message.set("Le lien de réinitialisation est invalide ou expiré.".to_string());
                        }
                    }
                    Err(_) => {
                        response_message.set("Erreur de connexion. Veuillez réessayer.".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="container mt-5">
            <div class="row justify-content-center">
                <div class="col-md-6">
                    <div class="card">
                        <div class="card-body">
                            <h1 class="card-title text-center mb-4">{ "Réinitialiser le mot de passe" }</h1>
                            {
                                if *password_reset_success {
                                    html! {
                                        <div class="text-center">
                                            <i class="bi bi-check-circle-fill text-success" style="font-size: 3rem;"></i>
                                            <div class="alert alert-success mt-3" role="alert">
                                                { (*response_message).clone() }
                                            </div>
                                            <a href="#" class="btn btn-primary">
                                                { "Se connecter" }
                                            </a>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <>
                                            <p class="text-muted text-center mb-4">
                                                { "Choisissez un nouveau mot de passe sécurisé." }
                                            </p>
                                            <form onsubmit={on_submit}>
                                                <div class="mb-3">
                                                    <label class="form-label">{ "Nouveau mot de passe" }</label>
                                                    <input
                                                        type="password"
                                                        class="form-control"
                                                        placeholder="Au moins 8 caractères"
                                                        value={(*new_password).clone()}
                                                        disabled={*is_loading}
                                                        oninput={Callback::from(move |e: InputEvent| {
                                                            let input: HtmlInputElement = e.target_unchecked_into();
                                                            new_password.set(input.value());
                                                        })}
                                                    />
                                                </div>
                                                <div class="mb-3">
                                                    <label class="form-label">{ "Confirmer le mot de passe" }</label>
                                                    <input
                                                        type="password"
                                                        class="form-control"
                                                        placeholder="Répétez le mot de passe"
                                                        value={(*confirm_password).clone()}
                                                        disabled={*is_loading}
                                                        oninput={Callback::from(move |e: InputEvent| {
                                                            let input: HtmlInputElement = e.target_unchecked_into();
                                                            confirm_password.set(input.value());
                                                        })}
                                                    />
                                                </div>
                                                <div class="d-grid mb-3">
                                                    <button 
                                                        type="submit" 
                                                        class="btn btn-primary"
                                                        disabled={*is_loading}
                                                    >
                                                        {
                                                            if *is_loading {
                                                                html! {
                                                                    <>
                                                                        <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                                                        { "Réinitialisation..." }
                                                                    </>
                                                                }
                                                            } else {
                                                                html! { "Réinitialiser le mot de passe" }
                                                            }
                                                        }
                                                    </button>
                                                </div>
                                            </form>
                                            {
                                                if !(*response_message).is_empty() && !*password_reset_success {
                                                    html! {
                                                        <div class="alert alert-danger mt-3" role="alert">
                                                            { (*response_message).clone() }
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </>
                                    }
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
