use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use crate::context::auth::use_auth;
use crate::types::Expense;
use crate::pages::expenses_list::ExpenseListComponent;
use crate::components::expense_edit::EditExpenseModal;
use crate::components::layout::Route;

#[function_component(ManageExpenses)]
pub fn manage_expenses() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let expenses = use_state(|| vec![] as Vec<Expense>);
    let show_edit_modal = use_state(|| false);
    let edit_expense = use_state(|| None::<Expense>);

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

    // Delete expense
    let on_delete = {
        let expenses = expenses.clone();
        let auth = auth.clone();
        Callback::from(move |id: i32| {
            let expenses = expenses.clone();
            let auth = auth.clone();
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
                    <div class="col-12">
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <h2>{ "Gestion des dépenses" }</h2>
                            <Link<Route> to={Route::AddExpense} classes="btn btn-primary">
                                <i class="bi bi-plus-circle"></i>{ " Ajouter une dépense" }
                            </Link<Route>>
                        </div>
                        
                        <div class="card">
                            <div class="card-header">
                                <h5 class="mb-0">{ format!("Mes dépenses ({} au total)", expenses.len()) }</h5>
                            </div>
                            <div class="card-body">
                                <ExpenseListComponent
                                    expenses={(*expenses).clone()}
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
        </>
    }
}
