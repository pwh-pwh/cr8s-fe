use crate::components::header::Header;
use crate::components::rustacean_form::RustaceanForm;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::hooks::use_rustacean;
use crate::Route;
use yew::suspense::Suspense;
use yew::{function_component, html, use_context, AttrValue, Html, HtmlResult, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub rustacean_id: i32,
}

#[function_component(RustaceansEdit)]
pub fn rustaceans_edit(props: &Props) -> Html {
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
                                <RustaceanEditForm rustacean_id={props.rustacean_id} token={token.clone()} />
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

#[derive(Properties, PartialEq)]
struct RustaceanEditFormProps {
    pub rustacean_id: i32,
    pub token: AttrValue,
}

#[function_component(RustaceanEditForm)]
fn rustacean_edit_form(props: &RustaceanEditFormProps) -> HtmlResult {
    let rustacean = use_rustacean(&props.token, props.rustacean_id)?;
    Ok(html! {
        <RustaceanForm rustacean={rustacean} />
    })
}
