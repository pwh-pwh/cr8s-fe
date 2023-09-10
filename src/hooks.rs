use crate::api::rustaceans::{api_rustaceans, Rustacean};
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew::use_state_eq;

#[hook]
pub fn use_rustaceans(token: &String) -> SuspensionResult<Vec<Rustacean>> {
    let result_handler = use_state_eq(|| None);
    let result = (*result_handler).clone();
    let suspension_handle = use_state(|| {
        let token = token.clone();
        Suspension::from_future(async move {
            match api_rustaceans(&token).await {
                Ok(rustaceans) => result_handler.set(Some(rustaceans)),
                Err(_) => result_handler.set(Some(vec![])),
            }
        })
    });
    let suspension = (*suspension_handle).clone();
    if suspension.resumed() {
        match result {
            Some(v) => Ok(v),
            None => Err(suspension),
        }
    } else {
        Err(suspension)
    }
}
