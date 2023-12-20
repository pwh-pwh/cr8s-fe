use crate::api::rustaceans::Rustacean;
use crate::api::APP_HOST;
use crate::pages::crates::index::Crates;
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;
use std::process::id;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: String,
}

pub async fn api_crates(token: &String) -> Result<Vec<Crate>, gloo_net::Error> {
    let response = Request::get(&format!("{}/crates", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;
    response.json::<Vec<Crate>>().await
}

pub async fn api_crate_create(
    token: &String,
    name: String,
    code: String,
    version: String,
    description: String,
    rustacean_id: i32,
) -> Result<Crate, gloo_net::Error> {
    let response = Request::post(&format!("{}/crates", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name":name,
            "code":code,
            "version":version,
            "rustacean_id":rustacean_id,
            "description":description
        }))?
        .send()
        .await?;
    response.json::<Crate>().await
}

pub async fn api_crate_update(
    token: &String,
    name: String,
    code: String,
    version: String,
    description: String,
    rustacean_id: i32,
    id: i32,
) -> Result<Crate, gloo_net::Error> {
    let response = Request::put(&format!("{}/crates/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "name":name,
            "code":code,
            "version":version,
            "description":description,
            "rustacean_id":rustacean_id
        }))?
        .send()
        .await?;
    response.json::<Crate>().await
}

pub async fn api_crate_show(token: &String, id: i32) -> Result<Crate, gloo_net::Error> {
    let response = Request::get(&format!("{}/crates/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;
    response.json::<Crate>().await
}

pub async fn api_crate_delete(
    token:&String,
    id:i32
) ->Result<(),gloo_net::Error> {
    let _ = Request::delete(&format!("{}/crates/{}", APP_HOST, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;
    Ok(())
}