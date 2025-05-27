use yew::prelude::*;
use crate::types::{Expense, ExpenseCategory};

#[derive(Properties, PartialEq)]
pub struct ExpenseListProps {
    pub expenses: Vec<Expense>,
    pub on_update_click: Callback<Expense>,
    pub on_delete: Callback<i32>,
}

#[function_component(ExpenseListComponent)]
pub fn expense_list_component(props: &ExpenseListProps) -> Html {
    if props.expenses.is_empty() {
        html! {
            <div class="text-center text-muted">
                <p>{ "Aucune dépense enregistrée" }</p>
            </div>
        }
    } else {
        html! {
            <div class="list-group" style="max-height: 400px; overflow-y: auto;">
                {
                    for props.expenses.iter().map(|expense| {
                        let exp = expense.clone();
                        let category_badge_class = match exp.category {
                            ExpenseCategory::Groceries => "bg-success",
                            ExpenseCategory::Leisure => "bg-primary",
                            ExpenseCategory::Electronics => "bg-info",
                            ExpenseCategory::Utilities => "bg-warning",
                            ExpenseCategory::Clothing => "bg-secondary",
                            ExpenseCategory::Health => "bg-danger",
                            ExpenseCategory::Others => "bg-dark",
                        };
                        let category_text = match exp.category {
                            ExpenseCategory::Groceries => "Alimentation",
                            ExpenseCategory::Leisure => "Loisirs",
                            ExpenseCategory::Electronics => "électronique",
                            ExpenseCategory::Utilities => "Factures",
                            ExpenseCategory::Clothing => "Vêtements",
                            ExpenseCategory::Health => "Santé",
                            ExpenseCategory::Others => "Autres",
                        };
                        html! {
                            <div class="list-group-item d-flex justify-content-between align-items-center">
                                <div>
                                    <h6 class="mb-1">{ exp.description.as_deref().unwrap_or("Sans description") }</h6>
                                    <p class="mb-1 text-success fw-bold">{ format!("{} €", exp.amount) }</p>
                                    <span class={format!("badge {}", category_badge_class)}>{ category_text }</span>
                                </div>
                                <div class="btn-group" role="group">
                                    <button 
                                        type="button"
                                        class="btn btn-outline-primary btn-sm"
                                        onclick={{
                                            let on_update_click = props.on_update_click.clone();
                                            let exp = exp.clone();
                                            Callback::from(move |_| on_update_click.emit(exp.clone()))
                                        }}
                                    >
                                        { "✏️" }
                                    </button>
                                    <button 
                                        type="button"
                                        class="btn btn-outline-danger btn-sm"
                                        onclick={{
                                            let on_delete = props.on_delete.clone();
                                            let id = exp.id;
                                            Callback::from(move |_| on_delete.emit(id))
                                        }}
                                    >
                                    {"🗑️" }
                                    </button>
                                </div>
                            </div>
                        }
                    })
                }
            </div>
        }
    }
}
