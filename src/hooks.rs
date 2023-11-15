use crate::api::crates::{api_crates, Crate};
use crate::api::rustaceans::{api_rustacean_show, api_rustaceans, Rustacean};
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew::use_state_eq;

#[hook]
pub fn use_rustaceans(token: &str) -> SuspensionResult<Vec<Rustacean>> {
    let result_handler = use_state_eq(|| None);
    let result = (*result_handler).clone();
    let suspension_handle = use_state(|| {
        let token = token.clone().to_owned();
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

#[hook]
pub fn use_rustacean(token: &str, id: i32) -> SuspensionResult<Rustacean> {
    let result_handler = use_state_eq(|| None);
    let result = (*result_handler).clone();
    let suspension_handle = use_state(|| {
        let token = token.clone().to_owned();
        Suspension::from_future(async move {
            match api_rustacean_show(&token, id).await {
                Ok(rustacean) => result_handler.set(Some(rustacean)),
                Err(_) => result_handler.set(None),
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

#[hook]
pub fn use_crates(token: &str) -> SuspensionResult<Vec<Crate>> {
    let result_handler = use_state_eq(|| None);
    let result = (*result_handler).clone();
    let suspension_handle = use_state(|| {
        let token = token.clone().to_owned();
        Suspension::from_future(async move {
            match api_crates(&token).await {
                Ok(crates) => result_handler.set(Some(crates)),
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
