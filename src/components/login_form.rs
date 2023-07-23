use super::alert::Alert;
use super::input::Input;
use crate::api::user::{api_login, api_me, LoginResponse, MeResponse};
use crate::contexts::CurrentUserActions::LoginSuccess;
use crate::contexts::{CurrentDispatchActions, CurrentUserContext};
use crate::Route;
use gloo_console::log;
use web_sys::{Event, HtmlInputElement};
use yew::platform::spawn_local;
use yew::{
    function_component, html, use_context, use_state, Callback, Html, Properties, SubmitEvent,
    TargetCast,
};
use yew_router::prelude::use_navigator;

#[derive(PartialEq, Properties)]
pub struct LoginFormProps {}

async fn login(
    username: String,
    password: String,
) -> Result<(LoginResponse, MeResponse), gloo_net::Error> {
    let login_response = api_login(username, password).await?;
    let me_response = api_me(&login_response.token).await?;
    Ok((login_response, me_response))
}

#[function_component]
pub fn LoginForm(props: &LoginFormProps) -> Html {
    let navigator = use_navigator().unwrap();
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("current user context is missing");
    let username_handler = use_state(String::default);
    let username = (*username_handler).clone();
    let error_message_handler = use_state(String::default);
    let error_message = (*error_message_handler).clone();
    let username_changed = move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            username_handler.set(input.value());
        }
    };

    let password_handler = use_state(String::default);
    let password = (*password_handler).clone();
    let password_changed = move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            password_handler.set(input.value());
        }
    };
    let onsubmit = Callback::from(|e: SubmitEvent| {
        e.prevent_default();
    });
    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        // use move because username and password will out of the scope,so move,
        // use clone() because if not use username will move to log!() ,it be a FnOnce no Fn
        move |e: SubmitEvent| {
            e.prevent_default();
            log!("Submit from", username.clone(), password.clone());
            let current_user_ctx = current_user_ctx.clone();
            let username = username.clone();
            let password = password.clone();
            let navigator = navigator.clone();
            let error_message_handler = error_message_handler.clone();
            spawn_local(async move {
                match login(username, password).await {
                    Ok(response) => {
                        //log!(response.1.username);
                        current_user_ctx.dispatch(CurrentDispatchActions {
                            action_type: LoginSuccess,
                            login_response: Some(response.0),
                            me_response: Some(response.1),
                        });
                        navigator.push(&Route::Home);
                    }
                    Err(e) => error_message_handler.set(e.to_string()),
                }
            });
        }
    };
    html! {
        <>
        <form onsubmit={onsubmit}>
                    if error_message.len() != 0 {
                        <div>
                            <Alert message={error_message} alert_type="danger"/>
                        </div>
                    }
                    <div class="mb-3">
                        <Input name="username"
                        input_type="text"
                        value={username}
                        onchange={username_changed}
                        label="UserName"/>
                    </div>
                    <div class="mb-3">
                        <Input name="password"
                        input_type="password"
                        value={password}
                        onchange={password_changed}
                        label="Password"/>
                    </div>
                    <button type="submit" class="btn btn-primary">{"Login"}</button>
            </form>
        </>
    }
}
