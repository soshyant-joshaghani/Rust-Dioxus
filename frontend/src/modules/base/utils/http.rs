//! Thin reqwest wrapper. Feature `api.rs` files call these, never reqwest directly.
//! wasm32 uses the browser's fetch; native uses hyper + rustls.

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub use reqwest::Method;

use crate::config::api_url;
use crate::modules::base::utils::api_error::{format_api_error, ApiError};

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

fn request(token: Option<&str>, method: Method, path: &str) -> reqwest::RequestBuilder {
    let builder = client().request(method, api_url(path));
    match token {
        Some(token) => builder.bearer_auth(token),
        None => builder,
    }
}

fn network_error(err: reqwest::Error) -> ApiError {
    ApiError::new(format!("Unable to reach the API: {err}"), 0)
}

async fn check(res: reqwest::Response, fallback: &str) -> Result<reqwest::Response, ApiError> {
    if res.status().is_success() {
        return Ok(res);
    }
    let status = res.status().as_u16();
    let body: Value = res.json().await.unwrap_or(Value::Null);
    Err(ApiError::new(format_api_error(&body, &format!("{fallback} ({status})")), status))
}

async fn parse<T: DeserializeOwned>(res: reqwest::Response) -> Result<T, ApiError> {
    res.json::<T>()
        .await
        .map_err(|err| ApiError::new(format!("Invalid response from the API: {err}"), 0))
}

pub async fn get_json<T: DeserializeOwned>(token: Option<&str>, path: &str, fallback: &str) -> Result<T, ApiError> {
    let res = request(token, Method::GET, path).send().await.map_err(network_error)?;
    parse(check(res, fallback).await?).await
}

pub async fn send_json<B: Serialize, T: DeserializeOwned>(
    token: Option<&str>,
    method: Method,
    path: &str,
    body: &B,
    fallback: &str,
) -> Result<T, ApiError> {
    let res = request(token, method, path).json(body).send().await.map_err(network_error)?;
    parse(check(res, fallback).await?).await
}

pub async fn send_form<T: DeserializeOwned>(path: &str, form: &[(&str, &str)], fallback: &str) -> Result<T, ApiError> {
    let res = request(None, Method::POST, path).form(form).send().await.map_err(network_error)?;
    parse(check(res, fallback).await?).await
}

pub async fn delete(token: Option<&str>, path: &str, fallback: &str) -> Result<(), ApiError> {
    let res = request(token, Method::DELETE, path).send().await.map_err(network_error)?;
    check(res, fallback).await.map(|_| ())
}
