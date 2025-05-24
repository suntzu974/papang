use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::TargetCast;
use bigdecimal::BigDecimal;
use std::cmp::PartialEq;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginForm {
    email: String,
    password: String,
}
#[derive(Deserialize, Debug)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpenseCategory {
    Groceries,
    Leisure,
    Electronics,
    Utilities,
    Clothing,
    Health,
    Others,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub category: ExpenseCategory,
    pub amount: BigDecimal,
    pub description: Option<String>,
    pub expense_date: NaiveDateTime,
}

// Manual PartialEq implementation for Expense, comparing BigDecimal as string
impl PartialEq for Expense {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.category == other.category
            && self.amount.to_string() == other.amount.to_string()
            && self.description == other.description
            && self.expense_date == other.expense_date
    }
}
// Context pour le token d'authentification
#[derive(Clone, Debug, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
    pub set_token: Callback<Option<String>>,
    pub access_token: Option<String>, // Ajout du champ access_token
}

// Provider pour le contexte d'authentification
#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let token = use_state(|| None::<String>);
    let access_token = (*token).clone(); // Utilise le même token pour access_token, à adapter selon logique

    let set_token = {
        let token = token.clone();
        Callback::from(move |new_token: Option<String>| {
            token.set(new_token);
        })
    };

    let auth_context = AuthContext {
        token: (*token).clone(),
        set_token,
        access_token, // Ajout du champ access_token
    };

    html! {
        <ContextProvider<AuthContext> context={auth_context}>
            { for props.children.iter() }
        </ContextProvider<AuthContext>>
    }
}
// Hook personnalisé pour utiliser le contexte d'authentification
#[hook]
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not found")
}


// Composant de login
#[function_component(LoginComponent)]
fn login_component() -> Html {
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());
    let auth_context = use_auth();

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let response_message = response_message.clone();
        let set_token = auth_context.set_token.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = email.clone();
            let password = password.clone();
            let response_message = response_message.clone();
            let set_token = set_token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let login_data = LoginForm {
                    email: (*email).clone(),
                    password: (*password).clone(),
                };

                let res = Request::post("http://localhost:3001/auth/login")
                    .header("Content-Type", "application/json")
                    .json(&login_data)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) => {
                        if resp.status() == 200 {
                            let json = resp.json::<LoginResponse>().await.unwrap();
                            let access_token = json.access_token.clone();
                            set_token.emit(Some(access_token));
                            response_message.set("Login réussi".to_string());
                        } else if resp.status() == 401 {
                            response_message.set("Identifiants invalides".to_string());
                        } else {
                            response_message.set("Erreur serveur".to_string());
                        }
                    }
                    Err(_) => {
                        response_message.set("Erreur de connexion".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Connexion" }</h1>
            <form onsubmit={on_submit}>
                <input
                    type="text"
                    placeholder="Nom d'utilisateur"
                    value={(*email).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        email.set(input.value());
                    })}
                />
                <br />
                <input
                    type="password"
                    placeholder="Mot de passe"
                    value={(*password).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        password.set(input.value());
                    })}
                />
                <br />
                <button type="submit">{ "Se connecter" }</button>
            </form>
            <p>{ (*response_message).clone() }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ExpenseListProps {
    pub expenses: Vec<Expense>,
}

#[function_component(ExpenseList)]
fn expense_list(props: &ExpenseListProps) -> Html {
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


// Point d'entrée principal avec le provider
#[function_component(Main)]
fn main_component() -> Html {
    let expenses = use_state(|| vec![] as Vec<Expense>);
    let token = use_state(|| None::<String>);

    let set_token = {
        let token = token.clone();
        Callback::from(move |new_token: Option<String>| {
            token.set(new_token);
        })
    };

    let fetch_expenses = {
        let token = token.clone();
        let expenses = expenses.clone();
        Callback::from(move |access_token: String| {
            let expenses = expenses.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let expenses_res = Request::get("http://localhost:3001/expenses")
                    .header("Authorization", &format!("Bearer {}", access_token))
                    .send()
                    .await;
                if let Ok(exp_resp) = expenses_res {
                    if exp_resp.status() == 200 {
                        if let Ok(exp_list) = exp_resp.json::<Vec<Expense>>().await {
                            expenses.set(exp_list);
                        }
                    }
                }
            });
        })
    };

    let auth = use_auth();

    html! {
        if auth.access_token.is_some() {
            <ExpenseComponent /> 
        } else {
            <LoginComponent /> 
        }
    }
}


#[derive(Serialize)]
struct RegisterForm {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    access_token: String,
    refresh_token: String,
}

#[function_component(RegisterComponent)]
fn register_component() -> Html {
    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let response_message = use_state(|| "".to_string());
    let access_token = use_state(|| None::<String>);
    let auth_context = use_auth();

    let on_submit = {
        let name = name.clone();
        let email = email.clone();
        let password = password.clone();
        let response_message = response_message.clone();
        let access_token = access_token.clone();
        let set_token = auth_context.set_token.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let email = email.clone();
            let password = password.clone();
            let response_message = response_message.clone();
            let access_token = access_token.clone();
            let set_token = set_token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let register_data = RegisterForm {
                    name: (*name).clone(),
                    email: (*email).clone(),
                    password: (*password).clone(),
                };

                let res = Request::post("http://localhost:3001/auth/register")
                    .header("Content-Type", "application/json")
                    .json(&register_data)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) => {
                        if resp.status() == 201 {
                            match resp.json::<RegisterResponse>().await {
                                Ok(json) => {
                                    access_token.set(Some(json.access_token.clone()));
                                    set_token.emit(Some(json.access_token.clone())); // Enregistre dans AuthContext
                                    response_message.set("Inscription réussie".to_string());
                                }
                                Err(_) => {
                                    response_message.set("Erreur lors de la récupération du token".to_string());
                                }
                            }
                        } else if resp.status() == 409 {
                            response_message.set("Utilisateur déjà existant".to_string());
                        } else {
                            response_message.set("Erreur serveur".to_string());
                        }
                    }
                    Err(_) => {
                        response_message.set("Erreur de connexion".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Inscription" }</h1>
            <form onsubmit={on_submit}>
                <input
                    type="text"
                    placeholder="Name"
                    value={(*name).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        name.set(input.value());
                    })}
                />
                <br />
                <input
                    type="text"
                    placeholder="Email"
                    value={(*email).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        email.set(input.value());
                    })}
                />
                <br />
                <input
                    type="password"
                    placeholder="Mot de passe"
                    value={(*password).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        password.set(input.value());
                    })}
                />
                <br />
                <button type="submit">{ "S'inscrire" }</button>
            </form>
            <p>{ (*response_message).clone() }</p>
            {
                if let Some(token) = &auth_context.access_token {
                    html! { <p>{ format!("Access token: {}", token) }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[function_component(Header)]
fn header() -> Html {
    let auth = use_auth();
    html! {
        <header style="padding: 1em; background: #222; color: #fff; text-align: center;">
            <h1>{ "Papang - Gestion des Dépenses" }</h1>
            <Navbar />
            {
                if let Some(token) = &auth.token {
                    html! {
                        <div style="margin-top: 1em; font-size: 0.9em; color: #b2ffb2;">
                            { format!("Access token: {}", token) }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </header>
    }
}

#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
        <nav style="margin-top: 1em;">
            <a href="#" style="color: #fff; margin: 0 1em; text-decoration: underline;">{ "Dépenses" }</a>
            // Ajoutez d'autres liens ici si besoin
        </nav>
    }
}

fn Footer() -> Html {
    html! {
        <footer style="padding: 1em; background: #222; color: #fff; text-align: center; position: fixed; width: 100%; bottom: 0;">
            <span>{ "© 2024 Papang" }</span>
        </footer>
    }
}

#[function_component(ExpenseComponent)]
fn expense_component() -> Html {
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

    // Update expense (simple version: only description and amount)
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



#[function_component(AppRoot)]
fn app_root() -> Html {
    html! {
        <AuthProvider>
            <Header />
            <main style="padding-bottom: 4em;">
                <Main />
            </main>
            { Footer() }
        </AuthProvider>
    }
}

fn main() {
    yew::Renderer::<AppRoot>::new().render();
}