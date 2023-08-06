use crate::api::rustaceans::api_rustaceans;
use crate::contexts::CurrentUserContext;
use crate::utils::time_utils::iso_8601_to_default_date_format;
use crate::Route;
use yew::platform::spawn_local;
use yew::{function_component, html, use_context, use_state, Html, Properties};
use yew_router::prelude::*;

#[function_component(RustaceanList)]
pub fn rustacean_list() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("No current user context found");
    let rustaceans_handler = use_state(Vec::new);
    let rustaceans = (*rustaceans_handler).clone();
    match &current_user_ctx.token {
        Some(token) => {
            let token = token.clone();
            spawn_local(async move {
                let resp = api_rustaceans(&token).await.unwrap();
                rustaceans_handler.set(resp);
            });

            html! {
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
                }
        }
        None => {
            html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
    }
}
