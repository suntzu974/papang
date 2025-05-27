use yew::prelude::*;
use gloo_net::http::Request;
use crate::context::auth::use_auth;
use crate::types::{Expense, ExpenseCategory};

#[function_component(ExpenseDashboard)]
pub fn expense_dashboard() -> Html {
    let expenses = use_state(|| vec![] as Vec<Expense>);
    let auth = use_auth();

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

    // Calculate totals
    let total_amount: f64 = expenses.iter()
        .map(|e| e.amount.to_string().parse::<f64>().unwrap_or(0.0))
        .sum();

    let category_totals: std::collections::HashMap<ExpenseCategory, f64> = {
        let mut totals = std::collections::HashMap::new();
        for expense in expenses.iter() {
            let amount = expense.amount.to_string().parse::<f64>().unwrap_or(0.0);
            *totals.entry(expense.category).or_insert(0.0) += amount;
        }
        totals
    };

    html! {
        <div class="container mt-4">
            <div class="row">
                <div class="col-12">
                    <h2 class="mb-4">{ "Tableau de bord des dépenses" }</h2>
                </div>
            </div>
            
            // Summary cards
            <div class="row mb-4">
                <div class="col-md-3">
                    <div class="card text-white bg-primary">
                        <div class="card-body">
                            <h5 class="card-title">{ "Total des dépenses" }</h5>
                            <h3>{ format!("{:.2} €", total_amount) }</h3>
                        </div>
                    </div>
                </div>
                <div class="col-md-3">
                    <div class="card text-white bg-success">
                        <div class="card-body">
                            <h5 class="card-title">{ "Nombre de dépenses" }</h5>
                            <h3>{ expenses.len() }</h3>
                        </div>
                    </div>
                </div>
                <div class="col-md-3">
                    <div class="card text-white bg-info">
                        <div class="card-body">
                            <h5 class="card-title">{ "Dépense moyenne" }</h5>
                            <h3>{ 
                                if expenses.is_empty() { 
                                    "0.00 €".to_string() 
                                } else { 
                                    format!("{:.2} €", total_amount / expenses.len() as f64) 
                                }
                            }</h3>
                        </div>
                    </div>
                </div>
                <div class="col-md-3">
                    <div class="card text-white bg-warning">
                        <div class="card-body">
                            <h5 class="card-title">{ "Catégories" }</h5>
                            <h3>{ category_totals.len() }</h3>
                        </div>
                    </div>
                </div>
            </div>

            // Category breakdown
            <div class="row">
                <div class="col-md-6">
                    <div class="card">
                        <div class="card-header">
                            <h5>{ "Répartition par catégorie" }</h5>
                        </div>
                        <div class="card-body">
                            {
                                for category_totals.iter().map(|(category, total)| {
                                    let category_name = match category {
                                        ExpenseCategory::Groceries => "Alimentation",
                                        ExpenseCategory::Leisure => "Loisirs",
                                        ExpenseCategory::Electronics => "Électronique",
                                        ExpenseCategory::Utilities => "Factures",
                                        ExpenseCategory::Clothing => "Vêtements",
                                        ExpenseCategory::Health => "Santé",
                                        ExpenseCategory::Others => "Autres",
                                    };
                                    let percentage = if total_amount > 0.0 { (total / total_amount) * 100.0 } else { 0.0 };
                                    
                                    html! {
                                        <div class="mb-3">
                                            <div class="d-flex justify-content-between">
                                                <span>{ category_name }</span>
                                                <span>{ format!("{:.2} € ({:.1}%)", total, percentage) }</span>
                                            </div>
                                            <div class="progress">
                                                <div 
                                                    class="progress-bar" 
                                                    role="progressbar" 
                                                    style={format!("width: {}%", percentage)}
                                                    aria-valuenow={percentage.to_string()}
                                                    aria-valuemin="0" 
                                                    aria-valuemax="100"
                                                ></div>
                                            </div>
                                        </div>
                                    }
                                })
                            }
                        </div>
                    </div>
                </div>
                
                <div class="col-md-6">
                    <div class="card">
                        <div class="card-header">
                            <h5>{ "Dépenses récentes" }</h5>
                        </div>
                        <div class="card-body">
                            <div class="list-group list-group-flush" style="max-height: 300px; overflow-y: auto;">
                                {
                                        for expenses.iter().take(5).map(|expense| {
                                            html! {
                                                <div class="list-group-item d-flex justify-content-between align-items-center">
                                                    <div>
                                                        <h6 class="mb-1">{ expense.description.as_deref().unwrap_or("Sans description") }</h6>
                                                        <small class="text-muted">{ format!("{:?}", expense.category) }</small>
                                                    </div>
                                                    <span class="badge bg-primary rounded-pill">{ format!("{} €", expense.amount) }</span>
                                                </div>
                                            }
                                        })
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
