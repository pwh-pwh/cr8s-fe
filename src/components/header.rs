use crate::contexts::{CurrentDispatchActions, CurrentUserActions, CurrentUserContext};
use crate::Route;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, Callback, Html};
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>().expect("");
    match &current_user_ctx.user {
        Some(user) => {
            let onclick = {
                let current_user_ctx = current_user_ctx.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    current_user_ctx.dispatch(CurrentDispatchActions {
                        action_type: CurrentUserActions::LoginFail,
                        login_response: None,
                        me_response: None,
                    })
                })
            };
            html! {
                <div class="text-end">
                <p>
                            <span class="pe-1">{"Welcome user "}{user.username.clone()}</span>
                            <button class="btn btn-danger" onclick={onclick}>{"Logout"}</button>
                </p>
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
