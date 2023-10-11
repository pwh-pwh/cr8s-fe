use crate::hooks::use_rustaceans;
use crate::utils::time_utils::iso_8601_to_default_date_format;
use crate::Route;
use yew::{function_component, html, AttrValue, Html, HtmlResult, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token: AttrValue,
}

#[function_component(RustaceanList)]
pub fn rustacean_list(props: &Props) -> HtmlResult {
    let rustaceans = use_rustaceans(&props.token)?;

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
                            <Link<Route> to={Route::RustaceansEdit {id: rustacean.id}} classes="link-secondary">
                                <button class="btn btn-primary">
                                    {"Edit"}
                                </button>
                            </Link<Route>>
                            <span class="mx-1">{"/"}</span>
                            <Link<Route> to={Route::RustaceansAdd} classes="link-danger">
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
