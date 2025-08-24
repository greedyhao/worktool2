// src-tauri/src/speed_test/udp.rs

use super::{SpeedTestConfig, SpeedTestResult};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::oneshot;
use tokio::time::{interval, Duration};
use tauri::Emitter;

// 启动UDP服务器
pub async fn start_server(
    config: super::SpeedTestConfig,
    app_handle: tauri::AppHandle,
) -> Result<super::SpeedTestHandle, Box<dyn std::error::Error>> {
    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    let socket = UdpSocket::bind(addr).await?;
    log::info!("UDP Speed Test Server listening on {}", addr);

    let config_arc = Arc::new(config);
    let (abort_tx, mut abort_rx) = oneshot::channel::<()>();
    let app_handle_clone = app_handle.clone();

    let handle = tokio::spawn(async move {
        let mut buf = vec![0; 65536]; // 足够大的缓冲区
        let payload = vec![0u8; config_arc.payload_size_kb * 1024];
        let mut client_stats: std::collections::HashMap<
            SocketAddr,
            (u64, std::time::Instant, std::time::Instant),
        > = std::collections::HashMap::new(); // (total_bytes, start_time, last_report_time)

        loop {
            tokio::select! {
                res = socket.recv_from(&mut buf) => {
                    match res {
                        Ok((len, src)) => {
                            let now = std::time::Instant::now();
                            let entry = client_stats.entry(src).or_insert((0, now, now));
                            entry.0 += len as u64; // 累加接收字节

                            // // 处理下载模式 (向客户端发送数据)
                            // if config_arc.mode == "DownloadOnly" {
                            //      if let Err(e) = socket.send_to(&payload, src).await {
                            //          log::error!("UDP send_to error: {}", e);
                            //          // 可能客户端已断开，移除统计
                            //          client_stats.remove(&src);
                            //      } else {
                            //          entry.0 += payload.len() as u64; // 也统计发送的字节
                            //      }
                            // }

                            // 检查是否需要报告
                            let elapsed_since_last_report = now.duration_since(entry.2).as_millis() as u64;
                            if elapsed_since_last_report >= config_arc.refresh_interval {
                                let elapsed_since_start = now.duration_since(entry.1).as_secs_f64();
                                let elapsed_since_last_report_secs = (elapsed_since_last_report as f64) / 1000.0;

                                if elapsed_since_last_report_secs > 0.0 {
                                    let speed_bps = (entry.0 as f64) / elapsed_since_last_report_secs;
                                    let speed_mbps = (speed_bps * 8.0) / 1_000_000.0;

                                    let result = SpeedTestResult {
                                        protocol: "UDP".to_string(),
                                        mode: config_arc.mode.clone(),
                                        client_address: src.to_string(),
                                        speed_mbps,
                                        latency_ms: 0.0, // UDP不直接测延迟
                                        message: "Running".to_string(),
                                        timestamp: std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis(),
                                    };
                                    if let Err(e) = app_handle_clone.emit("speed-test-result", result) {
                                        log::error!("Failed to emit UDP result: {}", e);
                                    }
                                    entry.2 = now; // 更新最后报告时间
                                    entry.0 = 0; // 重置计数器
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("UDP recv_from error: {}", e);
                        }
                    }
                }
                _ = &mut abort_rx => {
                    log::info!("UDP server received stop signal");
                    break;
                }
            }
        }
        // 可以在这里处理连接断开的最终统计，但UDP是无连接的，较难精确判断
    });

    Ok(super::SpeedTestHandle {
        abort_handle: abort_tx,
    })
}
