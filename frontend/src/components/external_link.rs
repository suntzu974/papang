use yew::prelude::*;
 use crate::services::url_service::UrlService;
use crate::context::auth::use_auth;

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
    pub href: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub new_tab: bool,
    #[prop_or(false)]
    pub validate_with_backend: bool,
}

#[function_component(ExternalLink)]
pub fn external_link(props: &ExternalLinkProps) -> Html {
    let auth = use_auth();
    let is_loading = use_state(|| false);

    let onclick = {
        let href = props.href.clone();
        let new_tab = props.new_tab;
        let validate_with_backend = props.validate_with_backend;
        let auth = auth.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let href = href.clone();
            let auth = auth.clone();
            let is_loading = is_loading.clone();

            if validate_with_backend {
                is_loading.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    match UrlService::validate_and_open_url(&href, new_tab, auth.token.as_deref()).await {
                        Ok(_) => {},
                        Err(err) => {
                            web_sys::console::error_1(&format!("Failed to open URL: {}", err).into());
                        }
                    }
                    is_loading.set(false);
                });
            } else {
                UrlService::open_external_link(&href);
            }
        })
    };

    html! {
        <a 
            href={props.href.clone()}
            class={props.class.clone()}
            onclick={onclick}
            target={if props.new_tab { "_blank" } else { "_self" }}
            rel={if props.new_tab { "noopener noreferrer" } else { "" }}
        >
            {
                if *is_loading {
                    html! {
                        <>
                            <span class="spinner-border spinner-border-sm me-1" role="status"></span>
                            { for props.children.iter() }
                        </>
                    }
                } else {
                    html! { for props.children.iter() }
                }
            }
        </a>
    }
}
