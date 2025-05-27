use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use crate::context::auth::use_auth;
use crate::types::{Expense, ExpenseCategory};
use crate::pages::expenses_list::ExpenseListComponent;
use crate::components::expense_edit::EditExpenseModal;
use crate::components::layout::Route;
use web_sys::{HtmlInputElement, console};

#[function_component(ManageExpenses)]
pub fn manage_expenses() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let expenses = use_state(|| vec![] as Vec<Expense>);
    let show_edit_modal = use_state(|| false);
    let edit_expense = use_state(|| None::<Expense>);
    let selected_category = use_state(|| None::<ExpenseCategory>);
    let confirm_delete_id = use_state(|| None::<i32>);

    // Redirect if not logged in
    if auth.token.is_none() {
        navigator.push(&Route::Login);
    }

    // Fetch expenses
    {
        let expenses = expenses.clone();
        let access_token = auth.access_token.clone();
        use_effect_with(
            access_token.clone(),
            move |access_token| {
                if let Some(token) = access_token {
                    let expenses = expenses.clone();
                    let token = token.clone();
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

    // Delete expense (trigger confirmation modal)
    let on_delete = {
        let confirm_delete_id = confirm_delete_id.clone();
        Callback::from(move |id: i32| {
            confirm_delete_id.set(Some(id));
        })
    };

    // Confirm delete handlers
    let on_confirm_delete = {
        let expenses = expenses.clone();
        let auth = auth.clone();
        let confirm_delete_id = confirm_delete_id.clone();
        Callback::from(move |_| {
            if let Some(id) = *confirm_delete_id {
                let expenses = expenses.clone();
                let auth = auth.clone();
                let confirm_delete_id = confirm_delete_id.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(token) = &auth.access_token {
                        let url = format!("http://localhost:3001/expenses/{}", id);
                        let res = Request::delete(&url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;
                        if let Ok(resp) = res {
                            if resp.status() == 204 {
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
                            }
                        }
                    }
                    confirm_delete_id.set(None);
                });
            }
        })
    };

    let on_cancel_delete = {
        let confirm_delete_id = confirm_delete_id.clone();
        Callback::from(move |_| confirm_delete_id.set(None))
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

    // Filter expenses by category
    let filtered_expenses = {
        let expenses = (*expenses).clone();
        let selected_category = (*selected_category).clone();
        
        if let Some(category) = selected_category {
            expenses.into_iter().filter(|e| e.category == category).collect::<Vec<_>>()
        } else {
            expenses
        }
    };

    let on_category_filter = {
        let selected_category = selected_category.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            let category = if value == "all" {
                None
            } else {
                Some(match value.as_str() {
                    "Groceries" => ExpenseCategory::Groceries,
                    "Leisure" => ExpenseCategory::Leisure,
                    "Electronics" => ExpenseCategory::Electronics,
                    "Utilities" => ExpenseCategory::Utilities,
                    "Clothing" => ExpenseCategory::Clothing,
                    "Health" => ExpenseCategory::Health,
                    _ => ExpenseCategory::Others,
                })
            };
            selected_category.set(category);
        })
    };

    html! {
        <>
            <div class="container mt-4">
                <div class="row">
                    <div class="col-12">
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <h2>{ "Gestion des dépenses" }</h2>
                            <Link<Route> to={Route::AddExpense} classes="btn btn-primary">
                                <i class="bi bi-plus-circle"></i>{ " Ajouter une dépense" }
                            </Link<Route>>
                        </div>
                        
                        <div class="card">
                            <div class="card-header d-flex justify-content-between align-items-center">
                                <h5 class="mb-0">{ format!("Mes dépenses ({} au total)", filtered_expenses.len()) }</h5>
                                <div class="d-flex align-items-center">
                                    <label class="form-label me-2 mb-0">{ "Filtrer par catégorie:" }</label>
                                    <select 
                                        class="form-select form-select-sm" 
                                        style="width: auto;"
                                        onchange={on_category_filter}
                                        value={
                                            if let Some(cat) = *selected_category {
                                                format!("{:?}", cat)
                                            } else {
                                                "all".to_string()
                                            }
                                        }
                                    >
                                        <option value="all">{ "Toutes les catégories" }</option>
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
                            <div class="card-body">
                                <ExpenseListComponent
                                    expenses={filtered_expenses}
                                    on_update_click={on_update_click.clone()}
                                    on_delete={on_delete.clone()}
                                />
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

            // Bootstrap confirmation modal
            {
                if let Some(_id) = *confirm_delete_id {
                    html! {
                        <div class="modal fade show" tabindex="-1" style="display: block; background: rgba(0,0,0,0.5);" aria-modal="true" role="dialog">
                            <div class="modal-dialog">
                                <div class="modal-content">
                                    <div class="modal-header">
                                        <h5 class="modal-title">{ "Confirmation de suppression" }</h5>
                                        <button type="button" class="btn-close" onclick={on_cancel_delete.clone()}></button>
                                    </div>
                                    <div class="modal-body">
                                        <p>{ "Voulez-vous vraiment supprimer cette dépense ?" }</p>
                                        <p class="text-muted">{ "Cette action est irréversible." }</p>
                                    </div>
                                    <div class="modal-footer">
                                        <button type="button" class="btn btn-secondary" onclick={on_cancel_delete}>
                                            { "Annuler" }
                                        </button>
                                        <button type="button" class="btn btn-danger" onclick={on_confirm_delete}>
                                            <i class="bi bi-trash"></i>{ " Supprimer" }
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
