use yew::prelude::*;
use crate::types::Expense;

#[derive(Properties, PartialEq)]
pub struct ExpenseListProps {
    pub expenses: Vec<Expense>,
}

#[function_component(ExpenseList)]
pub fn expense_list(props: &ExpenseListProps) -> Html {
    html! {
        <ul>
            {
                for props.expenses.iter().map(|expense| html! {
                    <li>
                        { format!(
                            "{}: {}€ ({:?})",
                            expense.description.as_deref().unwrap_or(""),
                            expense.amount,
                            expense.category
                        ) }
                    </li>
                })
            }
        </ul>
    }
}
