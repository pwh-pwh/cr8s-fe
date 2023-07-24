use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::Route;
use yew::{function_component, html, use_context, Html, Properties};
use yew_router::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("current user context is missing");
    match &current_user_ctx.user {
        Some(user) => {
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col">
                            <Sidebar />
                        </div>
                        <div class="col">
                            {"Welcome user "}{user.username.clone()}
                        </div>
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <Redirect<Route> to={Route::Login}/>
            }
        }
    }
}
