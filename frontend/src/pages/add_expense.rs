use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use serde::Serialize;
use crate::context::auth::use_auth;
use crate::types::ExpenseCategory;
use crate::components::layout::Route;
use crate::services::api_service::ApiService;

#[derive(Serialize)]
struct NewExpense<'a> {
    description: &'a str,
    amount: &'a str,
    category: ExpenseCategory,
}

#[function_component(AddExpense)]
pub fn add_expense() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let description = use_state(|| "".to_string());
    let amount = use_state(|| "".to_string());
    let category = use_state(|| ExpenseCategory::Others);
    let response_message = use_state(|| "".to_string());

    // Redirect if not logged in
    if auth.token.is_none() {
        navigator.push(&Route::Login);
    }

    let on_create = {
        let description = description.clone();
        let amount = amount.clone();
        let category = category.clone();
        let response_message = response_message.clone();
        let auth = auth.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let description = description.clone();
            let amount = amount.clone();
            let category = category.clone();
            let response_message = response_message.clone();
            let auth = auth.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth.access_token {
                    let new_expense = NewExpense {
                        description: &description,
                        amount: &amount,
                        category: *category,
                    };
                    let res = ApiService::post("/expenses")
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("Content-Type", "application/json")
                        .json(&new_expense)
                        .unwrap()
                        .send()
                        .await;
                    match res {
                        Ok(resp) => {
                            if resp.status() == 201 {
                                response_message.set("Dépense ajoutée avec succès!".to_string());
                                description.set("".to_string());
                                amount.set("".to_string());
                                category.set(ExpenseCategory::Others);
                                // Redirect to manage expenses after 2 seconds
                                gloo::timers::callback::Timeout::new(2000, move || {
                                    navigator.push(&Route::ManageExpenses);
                                }).forget();
                            } else {
                                response_message.set("Erreur lors de l'ajout".to_string());
                            }
                        }
                        Err(_) => response_message.set("Erreur réseau".to_string()),
                    }
                }
            });
        })
    };

    html! {
        <div class="container-fluid">
            <div class="row justify-content-center">
                <div class="col-12 col-sm-10 col-md-8 col-lg-6 col-xl-5">
                    <div class="card shadow-sm">
                        <div class="card-header bg-primary text-white">
                            <h4 class="card-title mb-0 text-center">{ "Ajouter une nouvelle dépense" }</h4>
                        </div>
                        <div class="card-body p-4">
                            <form onsubmit={on_create}>
                                <div class="row">
                                    <div class="col-12">
                                        <div class="mb-3">
                                            <label class="form-label fw-semibold">{ "Description" }</label>
                                            <input
                                                type="text"
                                                class="form-control form-control-lg"
                                                placeholder="Description de la dépense"
                                                value={(*description).clone()}
                                                required=true
                                                oninput={{
                                                    let description = description.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        description.set(input.value());
                                                    })
                                                }}
                                            />
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="row">
                                    <div class="col-12 col-sm-6">
                                        <div class="mb-3">
                                            <label class="form-label fw-semibold">{ "Montant (€)" }</label>
                                            <input
                                                type="number"
                                                step="0.01"
                                                class="form-control form-control-lg"
                                                placeholder="0.00"
                                                value={(*amount).clone()}
                                                required=true
                                                oninput={{
                                                    let amount = amount.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        amount.set(input.value());
                                                    })
                                                }}
                                            />
                                        </div>
                                    </div>
                                    <div class="col-12 col-sm-6">
                                        <div class="mb-3">
                                            <label class="form-label fw-semibold">{ "Catégorie" }</label>
                                            <select
                                                class="form-select form-select-lg"
                                                value={format!("{:?}", *category)}
                                                onchange={{
                                                    let category = category.clone();
                                                    Callback::from(move |e: Event| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        let value = input.value();
                                                        let cat = match value.as_str() {
                                                            "Groceries" => ExpenseCategory::Groceries,
                                                            "Leisure" => ExpenseCategory::Leisure,
                                                            "Electronics" => ExpenseCategory::Electronics,
                                                            "Utilities" => ExpenseCategory::Utilities,
                                                            "Clothing" => ExpenseCategory::Clothing,
                                                            "Health" => ExpenseCategory::Health,
                                                            _ => ExpenseCategory::Others,
                                                        };
                                                        category.set(cat);
                                                    })
                                                }}
                                            >
                                                <option value="Groceries">{ "Alimentation" }</option>
                                                <option value="Leisure">{ "Loisirs" }</option>
                                                <option value="Electronics">{ "Électronique" }</option>
                                                <option value="Utilities">{ "Factures" }</option>
                                                <option value="Clothing">{ "Vêtements" }</option>
                                                <option value="Health">{ "Santé" }</option>
                                                <option value="Others">{ "Autres" }</option>
                                            </select>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="row">
                                    <div class="col-12">
                                        <div class="d-grid gap-3">
                                            <button type="submit" class="btn btn-primary btn-lg">
                                                <i class="bi bi-plus-circle me-2"></i>{ "Ajouter la dépense" }
                                            </button>
                                            <Link<Route> to={Route::ManageExpenses} classes="btn btn-outline-secondary">
                                                <i class="bi bi-arrow-left me-2"></i>{ "Retour à la liste" }
                                            </Link<Route>>
                                        </div>
                                    </div>
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
