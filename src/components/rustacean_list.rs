use crate::api::rustaceans::api_rustaceans;
use crate::contexts::CurrentUserContext;
use crate::hooks::use_rustaceans;
use crate::utils::time_utils::iso_8601_to_default_date_format;
use crate::Route;
use yew::platform::spawn_local;
use yew::{
    function_component, html, use_context, use_state, use_state_eq, Html, HtmlResult, Properties,
};
use yew_router::prelude::*;

#[function_component(RustaceanList)]
pub fn rustacean_list() -> HtmlResult {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("No current user context found");
    let token = current_user_ctx.token.as_ref().expect("");
    let rustaceans = use_rustaceans(token)?;

    Ok(html! {
        <>
            <Link<Route> to={Route::RustaceansAdd} >
                <button class="btn btn-primary">
                    {"+ Add new rustacean"}
                </button>
            </Link<Route>>
            <table class="table">
                <thead>
            <tr>
                <th>{ "ID" }</th>
                <th>{ "Name" }</th>
                <th>{ "Email" }</th>
                <th>{ "Created at" }</th>
                <th>{ "Operations" }</th>
            </tr>
            </thead>
        <tbody>
            {
                rustaceans.into_iter()
                .map(|rustacean| {
                html! {
                    <tr>
                        <td>{rustacean.id}</td>
                        <td>{rustacean.name}</td>
                        <td>{rustacean.email}</td>
                        <td>{iso_8601_to_default_date_format(&rustacean.created_at)}</td>
                        <td>
                            <Link<Route> to={Route::RustaceansAdd}>
                                <button class="btn btn-primary">
                                    {"Edit"}
                                </button>
                            </Link<Route>>
                            <Link<Route> to={Route::RustaceansAdd}>
                                <button class="btn btn-danger">
                                    {"Delete"}
                                </button>
                            </Link<Route>>
                        </td>
                    </tr>
                }
            }).collect::<Html>()
            }
        </tbody>
    </table>
        </>
        })
}
