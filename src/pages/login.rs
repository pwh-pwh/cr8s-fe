use super::super::components::login_form::LoginForm;
use crate::contexts::CurrentUserContext;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>().expect("");
    match &current_user_ctx.user {
        Some(_) => {
            html! {
                 <Redirect<Route> to={Route::Home} />
            }
        }
        None => {
            html! {
                <div class="container">
                    <div class="row min-vh-100 justify-content-center align-items-center">
                        <div class="col-md-4">
                            <p class="text-center">
                            <img src="./img.png" alt="logo" />
                            </p>
                            <LoginForm />
                        </div>
                    </div>
                </div>
            }
        }
    }
}
