use once_cell::sync::Lazy;
use reqwest::{Client, Method, Response, header};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .tcp_keepalive(Duration::from_secs(30))
        .pool_idle_timeout(Duration::from_secs(60))
        .gzip(true)
        .brotli(true)
        .build()
        .expect("Failed to create HTTP client")
});

pub async fn execute_request(
    method: Method,
    url: &str,
    body: Option<Value>,
    headers: HashMap<String, String>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let mut request = CLIENT.request(method, url);

    if !headers.is_empty() {
        let mut header_map = header::HeaderMap::with_capacity(headers.len());
        for (k, v) in headers {
            if let (Ok(name), Ok(value)) = (
                header::HeaderName::from_bytes(k.as_bytes()),
                header::HeaderValue::from_str(&v),
            ) {
                header_map.insert(name, value);
            }
        }
        request = request.headers(header_map);
    }

    if let Some(body) = body {
        request = request.json(&body);
    }

    let response = request.send().await?;
    Ok(response)
}
