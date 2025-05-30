use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use serde::Serialize;

#[derive(Serialize)]
struct ForgotPasswordForm {
    email: String,
}

#[function_component(ForgotPasswordComponent)]
pub fn forgot_password_component() -> Html {
    let email = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());
    let is_loading = use_state(|| false);

    let on_submit = {
        let email = email.clone();
        let response_message = response_message.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = email.clone();
            let response_message = response_message.clone();
            let is_loading = is_loading.clone();

            if (*email).is_empty() {
                response_message.set("Veuillez saisir votre adresse email".to_string());
                return;
            }

            is_loading.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                let forgot_password_data = ForgotPasswordForm {
                    email: (*email).clone(),
                };

                let res = Request::post("http://localhost:3001/auth/forgot-password")
                    .header("Content-Type", "application/json")
                    .json(&forgot_password_data)
                    .unwrap()
                    .send()
                    .await;

                is_loading.set(false);

                match res {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            response_message.set("Si un compte avec cette adresse email existe, un lien de réinitialisation a été envoyé.".to_string());
                        } else {
                            response_message.set("Une erreur s'est produite. Veuillez réessayer.".to_string());
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
                <div class="col-md-12">
                    <div class="card">
                        <div class="card-body">
                            <h1 class="card-title text-center mb-4">{ "Mot de passe oublié" }</h1>
                            <p class="text-muted text-center mb-4">
                                { "Saisissez votre adresse email et nous vous enverrons un lien pour réinitialiser votre mot de passe." }
                            </p>
                            <form onsubmit={on_submit}>
                                <div class="mb-3">
                                    <label class="form-label">{ "Adresse email" }</label>
                                    <input
                                        type="email"
                                        class="form-control"
                                        placeholder="votre@email.com"
                                        value={(*email).clone()}
                                        disabled={*is_loading}
                                        oninput={Callback::from(move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            email.set(input.value());
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
                                                        { "Envoi en cours..." }
                                                    </>
                                                }
                                            } else {
                                                html! { "Envoyer le lien de réinitialisation" }
                                            }
                                        }
                                    </button>
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
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
