// src-tauri/src/speed_test/mod.rs

// pub mod tcp;
pub mod udp;
pub mod websocket;

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// 定义从前端传入的配置
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedTestConfig {
    pub protocol: String, // "TCP", "UDP", "WebSocket"
    pub mode: String,     // "UploadOnly", "DownloadOnly"
    pub port: u16,
    pub refresh_interval: u64, // 毫秒
    pub payload_size_kb: usize, // KB
}

// 定义发送给前端的结果
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedTestResult {
    pub protocol: String,
    pub mode: String,
    pub client_address: String, // 格式: "IP:Port"
    pub speed_mbps: f64,
    pub latency_ms: f64,
    pub message: String,
    pub timestamp: u128, // Unix timestamp in milliseconds
}

// 定义测速任务的句柄，用于停止任务
pub struct SpeedTestHandle {
    pub abort_handle: tokio::sync::oneshot::Sender<()>,
}

// 定义测速模块的公共接口
pub async fn start_speed_test(
    config: SpeedTestConfig,
    app_handle: tauri::AppHandle,
) -> Result<SpeedTestHandle, Box<dyn std::error::Error>> {
    match config.protocol.as_str() {
        // "TCP" => tcp::start_server(config, app_handle).await,
        "UDP" => udp::start_server(config, app_handle).await,
        "WebSocket" => websocket::start_server(config, app_handle).await,
        _ => Err(format!("Unsupported protocol: {}", config.protocol).into()),
    }
}
