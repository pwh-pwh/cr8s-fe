use crate::contexts::CurrentUserContext;
use crate::Route;
use yew::{function_component, html, use_context, Html, Properties};
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("current user context is missing");
    match &current_user_ctx.user {
        Some(user) => {
            html! {
                <div class="container">
                    {"Welcome user "}{user.username.clone()}
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
