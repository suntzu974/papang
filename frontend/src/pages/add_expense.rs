use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use serde::Serialize;
use crate::context::auth::use_auth;
use crate::types::ExpenseCategory;
use crate::components::layout::Route;

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
                    let res = Request::post("http://localhost:3001/expenses")
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
        <div class="container mt-4">
            <div class="row justify-content-center">
                <div class="col-md-6">
                    <div class="card">
                        <div class="card-header">
                            <h3 class="card-title mb-0">{ "Ajouter une nouvelle dépense" }</h3>
                        </div>
                        <div class="card-body">
                            <form onsubmit={on_create}>
                                <div class="mb-3">
                                    <label class="form-label">{ "Description" }</label>
                                    <input
                                        type="text"
                                        class="form-control"
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
                                <div class="mb-3">
                                    <label class="form-label">{ "Montant (€)" }</label>
                                    <input
                                        type="number"
                                        step="0.01"
                                        class="form-control"
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
                                <div class="mb-3">
                                    <label class="form-label">{ "Catégorie" }</label>
                                    <select
                                        class="form-select"
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
                                <div class="d-grid gap-2">
                                    <button type="submit" class="btn btn-primary">
                                        <i class="bi bi-plus-circle"></i>{ " Ajouter la dépense" }
                                    </button>
                                    <Link<Route> to={Route::ManageExpenses} classes="btn btn-outline-secondary">
                                        { "Retour à la liste" }
                                    </Link<Route>>
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
