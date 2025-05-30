use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use serde::Serialize;
use crate::context::auth::{use_auth, check_auth_response};
use crate::types::{Expense, ExpenseCategory};
use crate::components::expense_edit::EditExpenseModal;
use yew_router::prelude::*;
use crate::{Route};
use crate::pages::expenses_list::ExpenseListComponent; // Ajout de l'import
use crate::services::api_service::ApiService;

#[function_component(ExpenseComponent)]
pub fn expense_component() -> Html {
    let expenses = use_state(|| vec![] as Vec<Expense>);
    let description = use_state(|| "".to_string());
    let amount = use_state(|| "".to_string());
    let category = use_state(|| ExpenseCategory::Others);
    let response_message = use_state(|| "".to_string());
    let show_edit_modal = use_state(|| false);
    let edit_expense = use_state(|| None::<Expense>);
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    // Redirect if not logged in
    if auth.token.is_none() {
        navigator.push(&Route::Login);
    }

    // Fetch expenses on mount if token exists
    {
        let expenses = expenses.clone();
        let access_token = auth.access_token.clone();
        let auth_for_effect = auth.clone();
        use_effect_with(
            access_token.clone(),
            move |access_token| {
                let token_opt = access_token.clone();
                if let Some(token) = token_opt {
                    let expenses = expenses.clone();
                    let auth = auth_for_effect.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let res = ApiService::get("/expenses")
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;
                        if let Ok(resp) = res {
                            if check_auth_response(resp.status(), &auth) && resp.status() == 200 {
                                if let Ok(list) = resp.json::<Vec<Expense>>().await {
                                    expenses.set(list);
                                }
                            }
                        }
                    });
                }
                || ()
            }
        );
    }

    // Create expense
    let on_create = {
        let description = description.clone();
        let amount = amount.clone();
        let category = category.clone();
        let expenses = expenses.clone();
        let response_message = response_message.clone();
        let auth = auth.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let description = description.clone();
            let amount = amount.clone();
            let category = category.clone();
            let expenses = expenses.clone();
            let response_message = response_message.clone();
            let auth = auth.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth.access_token {
                    #[derive(Serialize)]
                    struct NewExpense<'a> {
                        description: &'a str,
                        amount: &'a str,
                        category: ExpenseCategory,
                    }
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
                            if check_auth_response(resp.status(), &auth) {
                                if resp.status() == 201 {
                                    response_message.set("Dépense ajoutée".to_string());
                                    // Clear the form
                                    description.set("".to_string());
                                    amount.set("".to_string());
                                    category.set(ExpenseCategory::Others);
                                    // Refresh list
                                    let res = Request::get("http://localhost:3001/expenses")
                                        .header("Authorization", &format!("Bearer {}", token))
                                        .send()
                                        .await;
                                    if let Ok(resp) = res {
                                        if resp.status() == 200 {
                                            if let Ok(list) = resp.json::<Vec<Expense>>().await {
                                                expenses.set(list);
                                            }
                                        }
                                    }
                                } else {
                                    response_message.set("Erreur lors de l'ajout".to_string());
                                }
                            }
                        }
                        Err(_) => response_message.set("Erreur réseau".to_string()),
                    }
                }
            });
        })
    };

    // Delete expense
    let on_delete = {
        let expenses = expenses.clone();
        let response_message = response_message.clone();
        let auth = auth.clone();
        Callback::from(move |id: i32| {
            let expenses = expenses.clone();
            let response_message = response_message.clone();
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth.access_token {
                    let url = format!("http://localhost:3001/expenses/{}", id);
                    let res = Request::delete(&url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await;
                    match res {
                        Ok(resp) => {
                            if check_auth_response(resp.status(), &auth) {
                                if resp.status() == 204 {
                                    response_message.set("D�pense supprim�e".to_string());
                                    // Refresh list
                                    let res = Request::get("http://localhost:3001/expenses")
                                        .header("Authorization", &format!("Bearer {}", token))
                                        .send()
                                        .await;
                                    if let Ok(resp) = res {
                                        if resp.status() == 200 {
                                            if let Ok(list) = resp.json::<Vec<Expense>>().await {
                                                expenses.set(list);
                                            }
                                        }
                                    }
                                } else {
                                    response_message.set("Erreur lors de la suppression".to_string());
                                }
                            }
                        }
                        Err(_) => response_message.set("Erreur r�seau".to_string()),
                    }
                }
            });
        })
    };

    // Update expense
    let on_update_click = {
        let show_edit_modal = show_edit_modal.clone();
        let edit_expense = edit_expense.clone();
        Callback::from(move |expense: Expense| {
            edit_expense.set(Some(expense));
            show_edit_modal.set(true);
        })
    };

    let on_edit_close = {
        let show_edit_modal = show_edit_modal.clone();
        let edit_expense = edit_expense.clone();
        Callback::from(move |_| {
            show_edit_modal.set(false);
            edit_expense.set(None);
        })
    };

    let on_edit_update = {
        let expenses = expenses.clone();
        let response_message = response_message.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            let expenses = expenses.clone();
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth.access_token {
                    let res = Request::get("http://localhost:3001/expenses")
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await;
                    if let Ok(resp) = res {
                        if resp.status() == 200 {
                            if let Ok(list) = resp.json::<Vec<Expense>>().await {
                                expenses.set(list);
                            }
                        }
                    }
                }
            });
        })
    };

    html! {
        <>
            <div class="container mt-4">
                <div class="row">
                    <div class="col-md-6">
                        <div class="card">
                            <div class="card-header">
                                <h3 class="card-title mb-0">{ "Ajouter une dépense" }</h3>
                            </div>
                            <div class="card-body">
                                <form onsubmit={on_create}>
                                    <div class="mb-3">
                                        <input
                                            type="text"
                                            class="form-control"
                                            placeholder="Description"
                                            value={(*description).clone()}
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
                                        <input
                                            type="number"
                                            step="0.01"
                                            class="form-control"
                                            placeholder="Montant (€)"
                                            value={(*amount).clone()}
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
                                    <div class="d-grid">
                                        <button type="submit" class="btn btn-primary">{ "Ajouter" }</button>
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
                    <div class="col-md-6">
                        <div class="card">
                            <div class="card-header">
                                <h3 class="card-title mb-0">{ "Liste des dépenses" }</h3>
                            </div>
                            <div class="card-body">
                                {
                                    // Remplacer la logique d'affichage de la liste par le composant ExpenseListComponent
                                    html! {
                                        <ExpenseListComponent
                                            expenses={(*expenses).clone()}
                                            on_update_click={on_update_click.clone()}
                                            on_delete={on_delete.clone()}
                                        />
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            <EditExpenseModal
                expense={(*edit_expense).clone()}
                show={*show_edit_modal}
                on_close={on_edit_close}
                on_update={on_edit_update}
            />
        </>
    }
}
