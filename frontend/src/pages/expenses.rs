use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use serde::Serialize;
use crate::context::auth::use_auth;
use crate::types::{Expense, ExpenseCategory};

#[function_component(ExpenseComponent)]
pub fn expense_component() -> Html {
    let expenses = use_state(|| vec![] as Vec<Expense>);
    let description = use_state(|| "".to_string());
    let amount = use_state(|| "".to_string());
    let category = use_state(|| ExpenseCategory::Others);
    let response_message = use_state(|| "".to_string());
    let auth = use_auth();

    // Fetch expenses on mount if token exists
    {
        let expenses = expenses.clone();
        let access_token = auth.access_token.clone();
        use_effect_with(
            access_token.clone(),
            move |access_token| {
                let token_opt = access_token.clone();
                if let Some(token) = token_opt {
                    let expenses = expenses.clone();
                    wasm_bindgen_futures::spawn_local(async move {
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
                            if resp.status() == 201 {
                                response_message.set("Dépense ajoutée".to_string());
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
                            if resp.status() == 200 {
                                response_message.set("Dépense supprimée".to_string());
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
                        Err(_) => response_message.set("Erreur réseau".to_string()),
                    }
                }
            });
        })
    };

    // Update expense
    let on_update = {
        let expenses = expenses.clone();
        let response_message = response_message.clone();
        let auth = auth.clone();
        Callback::from(move |expense: Expense| {
            let expenses = expenses.clone();
            let response_message = response_message.clone();
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth.access_token {
                    let url = format!("http://localhost:3001/expenses/{}", expense.id);
                    let res = Request::put(&url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .header("Content-Type", "application/json")
                        .json(&expense)
                        .unwrap()
                        .send()
                        .await;
                    match res {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                response_message.set("Dépense modifiée".to_string());
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
                                response_message.set("Erreur lors de la modification".to_string());
                            }
                        }
                        Err(_) => response_message.set("Erreur réseau".to_string()),
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "Ajouter une dépense" }</h2>
            <form onsubmit={on_create}>
                <input
                    type="text"
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
                <input
                    type="text"
                    placeholder="Montant"
                    value={(*amount).clone()}
                    oninput={{
                        let amount = amount.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            amount.set(input.value());
                        })
                    }}
                />
                <select
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
                <button type="submit">{ "Ajouter" }</button>
            </form>
            <p>{ (*response_message).clone() }</p>
            <h2>{ "Liste des dépenses" }</h2>
            <ul>
                {
                    for expenses.iter().map(|expense| {
                        let exp = expense.clone();
                        html! {
                            <li>
                                { format!("{}: {}€ ({:?})", exp.description.as_deref().unwrap_or(""), exp.amount, exp.category) }
                                <button onclick={{
                                    let on_delete = on_delete.clone();
                                    let id = exp.id;
                                    Callback::from(move |_| on_delete.emit(id))
                                }}>{ "Supprimer" }</button>
                                <button onclick={{
                                    let on_update = on_update.clone();
                                    let exp = exp.clone();
                                    Callback::from(move |_| on_update.emit(exp.clone()))
                                }}>{ "Modifier" }</button>
                            </li>
                        }
                    })
                }
            </ul>
        </div>
    }
}
