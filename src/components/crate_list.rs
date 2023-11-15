use crate::hooks::use_crates;
use crate::utils::time_utils::iso_8601_to_default_date_format;
use crate::Route;
use yew::{function_component, html, AttrValue, Html, HtmlResult, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token: AttrValue,
}

#[function_component(CrateList)]
pub fn crate_list(props: &Props) -> HtmlResult {
    let crates = use_crates(&props.token)?;

    Ok(html! {
        <>
            <Link<Route> to={Route::CratesAdd} >
                <button class="btn btn-primary">
                    {"+ Add new crate"}
                </button>
            </Link<Route>>
            <table class="table">
                <thead>
            <tr>
                <th>{ "ID" }</th>
                <th>{"Code"}</th>
                <th>{ "Name" }</th>
                <th>{ "Rustacean ID" }</th>
                <th>{ "Version" }</th>
                <th>{ "Description" }</th>
                <th>{ "Created at" }</th>
                <th>{ "Operations" }</th>
            </tr>
            </thead>
        <tbody>
            {
                crates.into_iter()
                .map(|a_crate| {
                html! {
                    <tr>
                        <td>{a_crate.id}</td>
                        <td>{a_crate.code}</td>
                        <td>{a_crate.name}</td>
                        <td>{a_crate.rustacean_id}</td>
                        <td>{a_crate.version}</td>
                        <td>{a_crate.description}</td>
                        <td>{iso_8601_to_default_date_format(&a_crate.created_at)}</td>
                        <td>
                            <Link<Route> to={Route::CratesEdit {id: a_crate.id}} classes="link-secondary">
                                <button class="btn btn-primary">
                                    {"Edit"}
                                </button>
                            </Link<Route>>
                            <span class="mx-1">{"/"}</span>
                            <Link<Route> to={Route::CratesDelete {id: a_crate.id}} classes="link-danger">
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
