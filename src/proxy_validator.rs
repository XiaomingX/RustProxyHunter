// File: proxy_validator.rs
use reqwest::Proxy;
use reqwest::Client;

pub async fn validate_proxy(addr: &str) -> Option<String> {
    if let Ok(client) = Client::builder()
        .proxy(Proxy::http(addr).unwrap())
        .build()
    {
        let headers = [
            ("User-Agent", "Mozilla/5.0"),
            ("Accept", "text/html,application/xhtml+xml"),
        ];
        let mut req = client.get("http://www.baidu.com");
        for (key, value) in headers.iter() {
            req = req.header(*key, *value);
        }
        if let Ok(_) = req.send().await {
            Some("HTTP Proxy".to_string())
        } else {
            Some("HTTP (无法通过代理访问)".to_string())
        }
    } else {
        Some("HTTP (代理配置失败)".to_string())
    }
}
