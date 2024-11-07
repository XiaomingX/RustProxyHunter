mod scanner;
mod proxy_validator;

use scanner::{scan_port, generate_ip_range};
use proxy_validator::validate_proxy;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;
use tokio::io::{self, AsyncBufReadExt};
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    println!("请输入网段（格式: 192.168.1.0/24）:");
    let mut input = String::new();
    let mut stdin = io::BufReader::new(io::stdin());
    stdin.read_line(&mut input).await?;
    let input = input.trim();

    let (base_ip_str, mask_str) = input.split_once('/').unwrap_or(("192.168.1.0", "24"));
    let base_ip: Ipv4Addr = base_ip_str.parse()?;
    let subnet_mask: u8 = mask_str.parse()?;

    let ip_range = generate_ip_range(base_ip, subnet_mask);

    let ports_to_scan = vec![3128, 8080, 8888, 1080, 8000, 8001, 9050, 8081, 8118, 3129, 5000, 8119, 8110, 3124, 9999, 8443, 8088, 1081];

    println!("开始扫描 {} 的代理端口...", input);

    let mut tasks = Vec::new();
    for ip in &ip_range {
        for &port in &ports_to_scan {
            tasks.push(scan_port(IpAddr::V4(*ip), port));
        }
    }

    let results = join_all(tasks).await;

    println!("扫描结果:");
    println!("------------------------");

    let mut open_ports = 0;
    for result in results {
        if result.is_open {
            open_ports += 1;
            let protocol = if let Some(protocol) = &result.protocol {
                protocol.clone()
            } else {
                validate_proxy(&format!("{}:{}", result.ip, result.port)).await.unwrap_or("Unknown".to_string())
            };
            println!("IP {} - 端口 {:5} - 开放 - 协议: {}", result.ip, result.port, protocol);
        }
    }

    let duration = start_time.elapsed();
    
    println!("
扫描统计:");
    println!("------------------------");
    println!("扫描总IP数: {}", ip_range.len());
    println!("扫描端口数: {}", ports_to_scan.len());
    println!("发现开放端口: {}", open_ports);
    println!("扫描总用时: {:.2}秒", duration.as_secs_f64());
    println!("平均每个IP用时: {:.2}毫秒",
        (duration.as_millis() as f64) / (ip_range.len() as f64)
    );

    Ok(())
}
