use crate::api::rustaceans::{api_rustacean_create, api_rustacean_update, Rustacean};
use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::contexts::CurrentUserContext;
use crate::Route;
use gloo_console::log;
use web_sys::{Event, HtmlInputElement, SubmitEvent};
use yew::platform::spawn_local;
use yew::{
    function_component, html, use_context, use_state, Callback, Html, Properties, TargetCast,
};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub rustacean: Option<Rustacean>,
}
#[function_component(RustaceanForm)]
pub fn rustacean_form(props: &Props) -> Html {
    let name_handler = use_state(|| match &props.rustacean {
        Some(r) => r.name.clone(),
        None => String::default(),
    });
    let name = (*name_handler).clone();
    let name_changed = move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handler.set(input.value());
        }
    };
    let email_handler = use_state(|| match &props.rustacean {
        Some(r) => r.email.clone(),
        None => String::default(),
    });
    let email = (*email_handler).clone();
    let email_changed = move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            email_handler.set(input.value());
        }
    };
    let error_message_handler = use_state(String::default);
    let error_message = (*error_message_handler).clone();
    let current_user_ctx = use_context::<CurrentUserContext>().expect("not found ctx");
    let navigator = use_navigator().unwrap();
    let onsubmit = {
        let name = name.clone();
        let email = email.clone();
        let cloned_rustacean = props.rustacean.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let email = email.clone();
            let cloned_rustacean = cloned_rustacean.clone();
            let current_user_ctx = current_user_ctx.clone();
            let error_message_handler = error_message_handler.clone();
            let navigator = navigator.clone();
            match &current_user_ctx.token {
                Some(token) => {
                    let token = token.clone();
                    spawn_local(async move {
                        if let Some(rustacean) = cloned_rustacean {
                            match api_rustacean_update(&token, name, email, rustacean.id).await {
                                Ok(resp) => {
                                    log!("resp:", resp.id, " ,name:", resp.name);
                                    navigator.push(&Route::Rustaceans)
                                }
                                Err(_) => {
                                    error_message_handler
                                        .set("session has expired.please login again".to_string());
                                }
                            }
                        } else {
                            match api_rustacean_create(&token, name, email).await {
                                Ok(resp) => {
                                    log!("resp:", resp.id, " ,name:", resp.name);
                                    navigator.push(&Route::Rustaceans)
                                }
                                Err(_) => {
                                    error_message_handler
                                        .set("session has expired.please login again".to_string());
                                }
                            }
                        }
                    });
                }
                None => {}
            }
        })
    };
    html! {
        <form onsubmit={onsubmit}>
            if error_message.len() != 0 {
                        <div>
                            <Alert message={error_message} alert_type="danger"/>
                        </div>
                    }
            <div class="mb-3">
                        <Input name="name"
                        input_type="text"
                        value={name}
                        onchange={name_changed}
                        label="Name"/>
                    </div>
                    <div class="mb-3">
                        <Input name="email"
                        input_type="email"
                        value={email}
                        onchange={email_changed}
                        label="Email"/>
                    </div>
                    <button type="submit" class="btn btn-primary">{"Save"}</button>
        </form>
    }
}
