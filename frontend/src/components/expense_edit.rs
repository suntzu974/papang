use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::{HtmlInputElement, console};
use yew::TargetCast;
use crate::context::auth::{use_auth, check_auth_response};
use crate::types::{Expense, ExpenseCategory};
use crate::services::api_service::ApiService;

#[derive(Properties, PartialEq)]
pub struct EditExpenseModalProps {
    pub expense: Option<Expense>,
    pub show: bool,
    pub on_close: Callback<()>,
    pub on_update: Callback<()>,
}

#[function_component(EditExpenseModal)]
pub fn edit_expense_modal(props: &EditExpenseModalProps) -> Html {
    let description = use_state(|| "".to_string());
    let amount = use_state(|| "".to_string());
    let category = use_state(|| ExpenseCategory::Others);
    let response_message = use_state(|| "".to_string());
    let auth = use_auth();

    // Initialize form when expense changes
    {
        let description = description.clone();
        let amount = amount.clone();
        let category = category.clone();
        let expense = props.expense.clone();
        
        use_effect_with(expense, move |expense| {
            if let Some(exp) = expense {
                description.set(exp.description.clone().unwrap_or_default());
                amount.set(exp.amount.to_string());
                category.set(exp.category);
                console::log_1(&format!("Initialized category from record: {:?}", exp.category).into());
            }
            || ()
        });
    }

    let on_save = {
        let description = description.clone();
        let amount = amount.clone();
        let category = category.clone();
        let response_message = response_message.clone();
        let auth = auth.clone();
        let expense = props.expense.clone();
        let on_update = props.on_update.clone();
        let on_close = props.on_close.clone();

        Callback::from(move |_: MouseEvent| {
            console::log_1(&"on_save clicked".into());
            
            if let Some(exp) = &expense {
                let description = description.clone();
                let amount = amount.clone();
                let category = category.clone();
                let response_message = response_message.clone();
                let auth = auth.clone();
                let expense_id = exp.id;
                let on_update = on_update.clone();
                let on_close = on_close.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    console::log_1(&format!("auth.access_token: {:?}", auth.access_token).into());
                    
                    if let Some(token) = &auth.access_token {
                        let updated_expense = Expense {
                            id: expense_id,
                            description: if description.is_empty() { None } else { Some((*description).clone()) },
                            amount: (*amount).parse().unwrap_or_default(),
                            category: *category,
                            expense_date: chrono::Utc::now().naive_utc(),
                        };

                        let res = ApiService::put("/expenses")
                            .header("Authorization", &format!("Bearer {}", token))
                            .header("Content-Type", "application/json")
                            .json(&updated_expense)
                            .unwrap()
                            .send()
                            .await;

                        match res {
                            Ok(resp) => {
                                if check_auth_response(resp.status(), &auth) {
                                    if resp.status() == 200 {
                                        on_update.emit(());
                                        on_close.emit(());
                                    }
                                }
                            }
                            Err(_) => {},
                        }
                    } else {
                        console::log_1(&"No access_token found".into());
                        response_message.set("Token d'authentification manquant".to_string());
                    }
                });
            } else {
                console::log_1(&"No expense found".into());
            }
        })
    };

    if !props.show {
        return html! {};
    }

    html! {
        <div class="modal show d-block" tabindex="-1" style="background-color: rgba(0,0,0,0.5);">
            <div class="modal-dialog modal-dialog-centered modal-lg">
                <div class="modal-content">
                    <div class="modal-header bg-primary text-white">
                        <h5 class="modal-title">{ "Modifier la dépense" }</h5>
                        <button 
                            type="button" 
                            class="btn-close btn-close-white" 
                            onclick={{
                                let on_close = props.on_close.clone();
                                Callback::from(move |_| on_close.emit(()))
                            }}
                        ></button>
                    </div>
                    <div class="modal-body p-4">
                        <div class="row">
                            <div class="col-12">
                                <div class="mb-3">
                                    <label class="form-label fw-semibold">{ "Description" }</label>
                                    <input
                                        type="text"
                                        class="form-control form-control-lg"
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
                        
                        {
                            if !(*response_message).is_empty() {
                                html! {
                                    <div class="alert alert-info" role="alert">
                                        { (*response_message).clone() }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div class="modal-footer">
                        <div class="d-flex flex-column flex-sm-row gap-2 w-100">
                            <button 
                                type="button" 
                                class="btn btn-secondary flex-fill flex-sm-grow-0"
                                onclick={{
                                    let on_close = props.on_close.clone();
                                    Callback::from(move |_| on_close.emit(()))
                                }}
                            >
                                { "Annuler" }
                            </button>
                            <button 
                                type="button" 
                                class="btn btn-primary flex-fill flex-sm-grow-0"
                                onclick={on_save}
                            >
                                { "Sauvegarder" }
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
