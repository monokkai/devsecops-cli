use colored::*;
use reqwest::{Method, Response};
use serde_json::Value;

pub async fn execute_request(
    method: Method,
    url: &str,
    body: Option<Value>,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut request = client.request(method, url);

    if let Some(headers) = headers {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    if let Some(body) = body {
        request = request.json(&body);
    }

    Ok(request.send().await?)
}

pub fn print_response(response: Response) {
    println!(
        "{} {}",
        "Status:".bold(),
        response.status().to_string().blue()
    );

    let headers = response.headers();
    for (key, value) in headers.iter() {
        println!(
            "{}: {}",
            key.to_string().green(),
            value.to_str().unwrap().yellow()
        );
    }

    let body = response.text().unwrap();
    println!("{}", body);
}
