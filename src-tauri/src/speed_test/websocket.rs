// src-tauri/src/speed_test/websocket.rs

use super::{SpeedTestConfig, SpeedTestResult};
use futures_util::{SinkExt, StreamExt}; // 用于 WebSocket 的 send 和 next
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::Emitter;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tokio_tungstenite::{accept_async, tungstenite::Error as WsError};
use std::time::{Duration, Instant};
use tokio::time::interval;

// 处理单个 WebSocket 客户端连接 (异步)
async fn handle_client(
    stream: TcpStream,
    config: Arc<SpeedTestConfig>,
    app_handle: tauri::AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    // 获取客户端地址
    let client_addr = ws_stream.get_ref().peer_addr()?.to_string();
    log::info!("WebSocket client connected: {}", client_addr);

    // 拆分 WebSocket 流为发送者和接收者
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // let payload = vec![0u8; config.payload_size_kb * 1024];
    let start_time = Instant::now();
    let mut total_bytes = 0u64;
    let mut last_report_time = start_time;
    let report_interval = Duration::from_millis(config.refresh_interval);
    let mut report_interval = interval(report_interval);
    report_interval.tick().await;

    // 发送一个 Ping 用于后续可能的延迟计算（简化）
    let ping_start = Instant::now();
    if let Err(e) = ws_sender
        .send(tokio_tungstenite::tungstenite::Message::Ping(vec![].into()))
        .await
    {
        log::warn!("Failed to send initial ping to {}: {}", client_addr, e);
    }

    // 主循环：读取和写入数据
    loop {
        // 使用 tokio::select! 来同时处理接收消息和定时报告
        tokio::select! {
            // 从客户端接收消息
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(tokio_tungstenite::tungstenite::Message::Close(_))) => {
                        log::info!("WebSocket client closed connection: {}", client_addr);
                        break; // 客户端主动关闭
                    }
                    Some(Ok(tokio_tungstenite::tungstenite::Message::Pong(_))) => {
                        // 收到 Pong，计算延迟
                        let latency_ms = ping_start.elapsed().as_secs_f64() * 1000.0;
                        log::debug!("Ping-Pong latency for {}: {:.2}ms", client_addr, latency_ms);
                        // 注意：这里的延迟计算是针对初始 Ping 的，实际应用中可能需要更复杂的机制
                        // 例如，周期性发送 Ping 并计算最近一次的延迟。
                    }
                    Some(Ok(tokio_tungstenite::tungstenite::Message::Binary(data))) => {
                         if config.mode == "UploadOnly" {
                             total_bytes += data.len() as u64;
                         }
                         // 忽略文本消息或 Ping（除了上面处理的）
                         log::debug!("Received bin message from {}", client_addr);
                    }
                    Some(Ok(tokio_tungstenite::tungstenite::Message::Text(text))) => {
                         log::debug!("Received text message from {}", client_addr);
                    }
                    Some(Err(WsError::ConnectionClosed)) => {
                        log::info!("WebSocket connection closed by {}: {}", client_addr, WsError::ConnectionClosed);
                        break; // 连接已关闭
                    }
                    Some(Err(e)) => {
                        log::warn!("WebSocket error for {}: {}", client_addr, e);
                        break; // 发生错误，断开连接
                    }
                    None => {
                        log::info!("WebSocket stream ended for {}", client_addr);
                        break; // 流结束
                    }
                    _ => { }
                }
            }
            // // 处理下载模式 (向客户端发送数据)
            // _ = tokio::time::sleep(Duration::from_millis(10)), if config.mode == "DownloadOnly" => {
            //      // 为了避免无限循环发送，我们在这里稍微延迟一下
            //      // 或者可以更精确地基于时间间隔来发送
            //      if let Err(e) = ws_sender.send(tokio_tungstenite::tungstenite::Message::Binary(payload.clone())).await {
            //          log::error!("WebSocket send error to {}: {}", client_addr, e);
            //          break; // 发送失败，断开连接
            //      }
            //      total_bytes += payload.len() as u64;
            // }
            // 定时报告速度
            _ = report_interval.tick() => {
                let now = Instant::now();
                let elapsed_since_start = now.duration_since(start_time).as_secs_f64();
                let elapsed_since_last_report = now.duration_since(last_report_time).as_secs_f64();
                log::info!("Elapsed since start: {:.2}s", elapsed_since_start);

                if elapsed_since_last_report > 0.0 {
                    let speed_bps = (total_bytes as f64) / elapsed_since_last_report;
                    let speed_mbps = speed_bps / 1_000.0;
                    // 简化延迟计算，使用上次报告以来的时间作为参考（不准确，仅示意）
                    let latency_ms = elapsed_since_last_report * 1000.0 / 2.0; // 假设往返时间是间隔的一半

                    let result = SpeedTestResult {
                        protocol: "WebSocket".to_string(),
                        mode: config.mode.clone(),
                        client_address: client_addr.clone(),
                        speed_mbps,
                        latency_ms, // 使用简化计算的延迟
                        message: "Running".to_string(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    };

                    if let Err(e) = app_handle.emit("speed-test-result", result) {
                         log::error!("Failed to emit WebSocket result for {}: {}", client_addr, e);
                    }
                    last_report_time = now;
                    total_bytes = 0; // 重置计数器

                    // 发送下一个 Ping
                    // let ping_start = Instant::now(); // 如果要精确计算每个ping的延迟
                    // if let Err(e) = ws_sender.send(tokio_tungstenite::tungstenite::Message::Ping(vec![].into())).await {
                    //     log::warn!("Failed to send ping to {}: {}", client_addr, e);
                    // }
                }
            }
        }
    }

    // 连接断开，发送最终结果
    let elapsed_total = start_time.elapsed().as_secs_f64();
    let avg_speed_bps = if elapsed_total > 0.0 {
        (total_bytes as f64) / elapsed_total
    } else {
        0.0
    };
    let avg_speed_kbs = avg_speed_bps / 1_000.0;
    // 最终延迟设为 0 或 N/A，因为连接已断开
    let final_latency_ms = 0.0;

    let final_result = SpeedTestResult {
        protocol: "WebSocket".to_string(),
        mode: config.mode.clone(),
        client_address: client_addr,
        speed_mbps: avg_speed_kbs,
        latency_ms: final_latency_ms,
        message: "Client disconnected".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    };
    // 忽略发送错误，因为客户端可能已经断开
    let _ = app_handle.emit("speed-test-result", final_result);

    Ok(())
}

// 启动 WebSocket 服务器 (异步)
pub async fn start_server(
    config: super::SpeedTestConfig,
    app_handle: tauri::AppHandle,
) -> Result<super::SpeedTestHandle, Box<dyn std::error::Error>> {
    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    let listener = TcpListener::bind(addr).await?;
    log::info!("WebSocket Speed Test Server listening on {}", addr);

    let config_arc = Arc::new(config);
    let (abort_tx, abort_rx) = oneshot::channel::<()>();

    // 克隆 app_handle 用于 spawn 的任务
    let app_handle_server = app_handle.clone();

    while let Ok((stream, _)) = listener.accept().await {
        let config_clone = Arc::clone(&config_arc);
        let app_handle_clone = app_handle_server.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, config_clone, app_handle_clone).await {
                eprintln!("连接错误: {}", e);
            }
        });
    }
    Ok(super::SpeedTestHandle {
        abort_handle: abort_tx,
    })
}
