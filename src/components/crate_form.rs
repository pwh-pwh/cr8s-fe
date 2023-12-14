use crate::api::crates::{api_crate_create, api_crate_update, Crate};
use crate::api::rustaceans::Rustacean;
use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::components::select::Select;
use crate::contexts::CurrentUserContext;
use crate::Route;
use gloo_console::log;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, SubmitEvent};
use yew::platform::spawn_local;
use yew::{
    function_component, html, props, use_context, use_state, AttrValue, Callback, Html, Properties,
    TargetCast,
};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub a_crate: Option<Crate>,
    pub authors: Vec<Rustacean>,
}
#[function_component(CrateForm)]
pub fn crate_form(props: &Props) -> Html {
    let name_handler = use_state(|| match &props.a_crate {
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
    let code_handler = use_state(|| match &props.a_crate {
        Some(r) => r.code.clone(),
        None => String::default(),
    });
    let code = (*code_handler).clone();
    let code_changed = move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            code_handler.set(input.value());
        }
    };

    let rustacean_id_handler = use_state(|| match &props.a_crate {
        Some(r) => r.rustacean_id.to_string(),
        None => String::default(),
    });
    let rustacean_id = (*rustacean_id_handler).clone();
    let rustacean_id_changed = move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            rustacean_id_handler.set(input.value());
        }
    };

    let version_handler = use_state(|| match &props.a_crate {
        Some(r) => r.version.clone(),
        None => String::default(),
    });
    let version = (*version_handler).clone();
    let version_changed = move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            version_handler.set(input.value());
        }
    };

    let error_message_handler = use_state(String::default);
    let error_message = (*error_message_handler).clone();
    let current_user_ctx = use_context::<CurrentUserContext>().expect("not found ctx");
    let navigator = use_navigator().unwrap();
    let onsubmit = {
        let name = name.clone();
        let code = code.clone();
        let version = version.clone();
        let rustacean_id = rustacean_id.clone();
        let cloned_crate = props.a_crate.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let code = code.clone();
            let rustacean_id = rustacean_id.clone();
            let version = version.clone();
            let cloned_crate = cloned_crate.clone();
            let current_user_ctx = current_user_ctx.clone();
            let error_message_handler = error_message_handler.clone();
            let navigator = navigator.clone();
            match &current_user_ctx.token {
                Some(token) => {
                    let token = token.clone();
                    spawn_local(async move {
                        if let Some(rustacean) = cloned_crate {
                            match api_crate_update(&token, name, code, version, rustacean.id).await
                            {
                                Ok(resp) => {
                                    log!("resp:", resp.id, " ,name:", resp.name);
                                    navigator.push(&Route::Crates)
                                }
                                Err(_) => {
                                    error_message_handler
                                        .set("session has expired.please login again".to_string());
                                }
                            }
                        } else {
                            match api_crate_create(
                                &token,
                                name,
                                code,
                                version,
                                rustacean_id.parse().unwrap(),
                            )
                            .await
                            {
                                Ok(resp) => {
                                    log!("resp:", resp.id, " ,name:", resp.name);
                                    navigator.push(&Route::Crates)
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
    let options = props
        .authors
        .iter()
        .map(|a| {
            (
                AttrValue::from(a.id.to_string()),
                AttrValue::from(a.name.clone()),
            )
        })
        .collect::<Vec<(AttrValue, AttrValue)>>();
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
                        <Input name="code"
                        input_type="text"
                        value={code}
                        onchange={code_changed}
                        label="Code"/>
                    </div>
                    <div class="mb-3">
                        <Input name="version"
                        input_type="text"
                        value={version}
                        onchange={version_changed}
                        label="Version"/>
                    </div>
                    <div class="mb-3">
                        <Select name="author"
                                label="Author"
                                value={rustacean_id}
                                onchange={rustacean_id_changed}
                                options={options}
                        />
                    </div>

                    <button type="submit" class="btn btn-primary">{"Save"}</button>
        </form>
    }
}
