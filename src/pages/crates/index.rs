use crate::components::crate_list::CrateList;
use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::Route;
use yew::suspense::Suspense;
use yew::{function_component, html, use_context, Html};
use yew_router::prelude::*;

#[function_component(Crates)]
pub fn crates() -> Html {
    let current_user = use_context::<CurrentUserContext>().expect("not found current user");
    match &current_user.token {
        Some(token) => {
            let loading = html! {
                <>
                    <p>{"loading data"}</p>
                </>
            };
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            <Suspense fallback={loading}>
                                <CrateList token={token.clone()}/>
                            </Suspense>
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
