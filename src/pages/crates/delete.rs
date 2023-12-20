use crate::api::crates::api_crate_delete;
use crate::components::alert::Alert;
use crate::components::crate_form::CrateForm;
use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::hooks::{use_crate, use_rustacean, use_rustaceans};
use crate::Route;
use serde_json::de::Read;
use web_sys::MouseEvent;
use yew::platform::spawn_local;
use yew::{
    function_component, html, props, use_context, use_state, AttrValue, Html, HtmlResult,
    Properties,
};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub crate_id: i32,
}

#[function_component(CratesDelete)]
pub fn crates_delete(props: &Props) -> Html {
    let current_user = use_context::<CurrentUserContext>().expect("not found current user");
    let navigator = use_navigator().unwrap();
    let error_msg_hander = use_state(String::default);
    let error_msg = (*error_msg_hander).clone();
    match &current_user.token {
        Some(token) => {
            let cloned_token = token.to_owned();
            let id = props.crate_id;
            let onclick = move |e: MouseEvent| {
                e.prevent_default();
                let navigator = navigator.clone();
                let error_msg_hander = error_msg_hander.clone();
                let cloned_token = cloned_token.clone();
                spawn_local(async move {
                    match api_crate_delete(&cloned_token, id).await {
                        Ok(()) => navigator.push(&Route::Crates),
                        Err(e) => error_msg_hander.set(e.to_string()),
                    }
                })
            };
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                                if error_msg.len() != 0 {
                        <div>
                            <Alert message={error_msg} alert_type="danger"/>
                        </div>
                    }
                            <p>
                                {"are you sure you want to delete crate #"}
                                {props.crate_id.clone()}
                            </p>
                            <button class="btn btn-danger" onclick={onclick}>{"Delete"}</button>
                        </div>
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
    }
}
