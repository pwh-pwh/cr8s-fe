use crate::components::crate_form::CrateForm;
use crate::components::header::Header;
use crate::components::rustacean_form::RustaceanForm;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::hooks::{use_crate, use_rustacean, use_rustaceans};
use crate::Route;
use yew::suspense::Suspense;
use yew::{function_component, html, use_context, AttrValue, Html, HtmlResult, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub crate_id: i32,
}

#[function_component(CratesEdit)]
pub fn crates_edit(props: &Props) -> Html {
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
                                <CrateEditForm crate_id={props.crate_id} token={token.clone()} />
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
struct CrateEditFormProps {
    pub crate_id: i32,
    pub token: AttrValue,
}

#[function_component(CrateEditForm)]
fn crate_edit_form(props: &CrateEditFormProps) -> HtmlResult {
    let a_crate = use_crate(&props.token, props.crate_id)?;
    let rustaceans = use_rustaceans(&props.token)?;
    Ok(html! {
        <CrateForm a_crate={a_crate} authors={rustaceans}/>
    })
}
