use crate::cli::HttpAction;
use crate::modules::http::client;
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

pub async fn handle(action: HttpAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        HttpAction::Get { url, headers } => {
            let headers = parse_headers(headers);
            let response = client::execute_request(Method::GET, &url, None, headers).await?;
            client::print_response(response).await?;
        }
        HttpAction::Post { url, body, headers } => {
            let body = body.map(|b| json!(b));
            let headers = parse_headers(headers);
            let response = client::execute_request(Method::POST, &url, body, headers).await?;
            client::print_response(response).await?;
        }
        HttpAction::Put { url, body, headers } => {
            let body = body.map(|b| json!(b));
            let headers = parse_headers(headers);
            let response = client::execute_request(Method::PUT, &url, body, headers).await?;
            client::print_response(response).await?;
        }
        HttpAction::Patch { url, body, headers } => {
            let body = body.map(|b| json!(b));
            let headers = parse_headers(headers);
            let response = client::execute_request(Method::PATCH, &url, body, headers).await?;
            client::print_response(response).await?;
        }
        HttpAction::Delete { url, headers } => {
            let headers = parse_headers(headers);
            let response = client::execute_request(Method::DELETE, &url, None, headers).await?;
            client::print_response(response).await?;
        }
        HttpAction::Head { url, headers } => {
            let headers = parse_headers(headers);
            let response = client::execute_request(Method::HEAD, &url, None, headers).await?;
            client::print_response(response).await?;
        }
        HttpAction::Options { url, headers } => {
            let headers = parse_headers(headers);
            let response = client::execute_request(Method::OPTIONS, &url, None, headers).await?;
            client::print_response(response).await?;
        }
    }
    Ok(())
}

fn parse_headers(raw: Vec<String>) -> HashMap<String, String> {
    raw.iter()
        .filter_map(|h| {
            let mut parts = h.splitn(2, ':');
            Some((
                parts.next()?.trim().to_string(),
                parts.next()?.trim().to_string(),
            ))
        })
        .collect()
}
