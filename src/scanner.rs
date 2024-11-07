// File: scanner.rs
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use reqwest::Proxy;

#[derive(Debug)]
pub struct ScanResult {
    pub ip: IpAddr,
    pub port: u16,
    pub is_open: bool,
    pub protocol: Option<String>, // 识别协议类型（HTTP/HTTPS）
}

pub async fn scan_port(ip: IpAddr, port: u16) -> ScanResult {
    let timeout_duration = Duration::from_secs(1);
    let addr = format!("{}:{}", ip, port);

    let result = timeout(timeout_duration, TcpStream::connect(&addr)).await;

    if result.is_err() || result.as_ref().unwrap().is_err() {
        return ScanResult {
            ip,
            port,
            is_open: false,
            protocol: None,
        };
    }

    let mut stream = result.unwrap().unwrap();

    let request = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
    if let Err(_) = stream.write_all(request).await {
        return ScanResult {
            ip,
            port,
            is_open: true,
            protocol: None,
        };
    }

    let mut response = vec![0; 1024];
    if let Ok(n) = stream.read(&mut response).await {
        let response_text = String::from_utf8_lossy(&response[..n]);
        
        let protocol = if response_text.starts_with("HTTP/1.") {
            if let Ok(client) = reqwest::Client::builder()
                .proxy(Proxy::http(&addr).unwrap())
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
        } else if response_text.starts_with("\x16\x03") {
            Some("HTTPS".to_string())
        } else {
            None
        };

        return ScanResult {
            ip,
            port,
            is_open: true,
            protocol,
        };
    }

    ScanResult {
        ip,
        port,
        is_open: true,
        protocol: None,
    }
}

pub fn generate_ip_range(base_ip: Ipv4Addr, subnet_mask: u8) -> Vec<Ipv4Addr> {
    let ip_count = 1 << (32 - subnet_mask);
    (0..ip_count)
        .map(|i| Ipv4Addr::from(u32::from(base_ip) | i))
        .collect()
}