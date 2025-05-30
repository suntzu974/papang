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
                            <div class="list-group-item">
                                <div class="row align-items-center">
                                    <div class="col-12 col-sm-8 col-md-9">
                                        <div class="d-flex flex-column flex-sm-row align-items-start align-items-sm-center">
                                            <div class="flex-grow-1 mb-2 mb-sm-0">
                                                <h6 class="mb-1">{ exp.description.as_deref().unwrap_or("Sans description") }</h6>
                                                <div class="d-flex flex-column flex-sm-row align-items-start align-items-sm-center gap-2">
                                                    <span class="fw-bold text-success">{ format!("{} €", exp.amount) }</span>
                                                    <span class={format!("badge {} badge-sm", category_badge_class)}>{ category_text }</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="col-12 col-sm-4 col-md-3">
                                        <div class="btn-group w-100 w-sm-auto d-flex d-sm-inline-flex" role="group">
                                            <button 
                                                type="button"
                                                class="btn btn-outline-primary btn-sm flex-fill flex-sm-grow-0"
                                                title="Modifier"
                                                onclick={{
                                                    let on_update_click = props.on_update_click.clone();
                                                    let exp = exp.clone();
                                                    Callback::from(move |_| on_update_click.emit(exp.clone()))
                                                }}
                                            >
                                                <i class="bi bi-pencil"></i>
                                                <span class="d-none d-md-inline ms-1">{ "Modifier" }</span>
                                            </button>
                                            <button 
                                                type="button"
                                                class="btn btn-outline-danger btn-sm flex-fill flex-sm-grow-0"
                                                title="Supprimer"
                                                onclick={{
                                                    let on_delete = props.on_delete.clone();
                                                    let id = exp.id;
                                                    Callback::from(move |_| on_delete.emit(id))
                                                }}
                                            >
                                                <i class="bi bi-trash"></i>
                                                <span class="d-none d-md-inline ms-1">{ "Supprimer" }</span>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    })
                }
            </div>
        }
    }
}
