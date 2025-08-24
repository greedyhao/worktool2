// use super::{SpeedTestConfig, SpeedTestResult};
// use std::net::SocketAddr;
// use std::sync::Arc;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream};
// use tokio::sync::oneshot;
// use tokio::time::{interval, Duration};
// use tauri::Emitter;

// // 处理单个TCP客户端连接
// async fn handle_client(
//     mut socket: TcpStream,
//     config: Arc<SpeedTestConfig>,
//     app_handle: tauri::AppHandle,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let client_addr = socket.peer_addr()?;
//     log::info!("TCP client connected: {}", client_addr);

//     let payload = vec![0u8; config.payload_size_kb * 1024];
//     let start_time = std::time::Instant::now();
//     let mut total_bytes = 0u64;
//     let mut interval_timer = interval(Duration::from_millis(config.refresh_interval));
//     let mut last_report_time = start_time;

//     loop {
//         tokio::select! {
//             // 接收或发送数据
//             res = async {
//                 match config.mode.as_str() {
//                     "UploadOnly" => {
//                         // 服务器接收数据
//                         let mut buffer = vec![0u8; 4096];
//                         match socket.read(&mut buffer).await {
//                             Ok(0) => return Ok(()), // 客户端断开
//                             Ok(n) => total_bytes += n as u64,
//                             Err(e) => return Err(e.into()),
//                         }
//                     }
//                     // "DownloadOnly" => {
//                     //     // 服务器发送数据
//                     //     socket.write_all(&payload).await?;
//                     //     total_bytes += payload.len() as u64;
//                     // }
//                     _ => return Err("Invalid mode for TCP".into()),
//                 }
//                 Ok::<(), Box<dyn std::error::Error>>(())
//             } => {
//                 if res.is_err() {
//                     break; // 客户端断开或出错
//                 }
//             }
//             // 定时报告
//             _ = interval_timer.tick() => {
//                 let now = std::time::Instant::now();
//                 let elapsed_since_start = now.duration_since(start_time).as_secs_f64();
//                 let elapsed_since_last_report = now.duration_since(last_report_time).as_secs_f64();

//                 if elapsed_since_last_report > 0.0 {
//                     let speed_bps = (total_bytes as f64) / elapsed_since_last_report;
//                     let speed_mbps = (speed_bps * 8.0) / 1_000_000.0;

//                     let result = SpeedTestResult {
//                         protocol: "TCP".to_string(),
//                         mode: config.mode.clone(),
//                         client_address: client_addr.to_string(),
//                         speed_mbps,
//                         latency_ms: 0.0, // TCP不直接测延迟
//                         message: "Running".to_string(),
//                         timestamp: std::time::SystemTime::now()
//                             .duration_since(std::time::UNIX_EPOCH)
//                             .unwrap()
//                             .as_millis(),
//                     };

//                     // 发送事件到前端
//                     app_handle.emit("speed-test-result", result)?;
//                     last_report_time = now;
//                     total_bytes = 0; // 重置计数器
//                 }
//             }
//         }
//     }

//     // 连接断开，发送最终结果
//     let elapsed_total = start_time.elapsed().as_secs_f64();
//     let avg_speed_bps = if elapsed_total > 0.0 { (total_bytes as f64) / elapsed_total } else { 0.0 };
//     let avg_speed_mbps = (avg_speed_bps * 8.0) / 1_000_000.0;

//     let final_result = SpeedTestResult {
//         protocol: "TCP".to_string(),
//         mode: config.mode.clone(),
//         client_address: client_addr.to_string(),
//         speed_mbps: avg_speed_mbps,
//         latency_ms: 0.0,
//         message: "Client disconnected".to_string(),
//         timestamp: std::time::SystemTime::now()
//             .duration_since(std::time::UNIX_EPOCH)
//             .unwrap()
//             .as_millis(),
//     };
//     app_handle.emit("speed-test-result", final_result)?;

//     Ok(())
// }

// // 启动TCP服务器
// pub async fn start_server(
//     config: super::SpeedTestConfig,
//     app_handle: tauri::AppHandle,
// ) -> Result<super::SpeedTestHandle, Box<dyn std::error::Error>> {
//     let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
//     let listener = TcpListener::bind(addr).await?;
//     log::info!("TCP Speed Test Server listening on {}", addr);

//     let config_arc = Arc::new(config);
//     let (abort_tx, mut abort_rx) = oneshot::channel::<()>();

//     let handle = tokio::spawn(async move {
//         loop {
//             tokio::select! {
//                 accept_result = listener.accept() => {
//                     match accept_result {
//                         Ok((socket, _)) => {
//                              let config_clone = Arc::clone(&config_arc);
//                              let app_handle_clone = app_handle.clone();
//                              tokio::spawn(handle_client(socket, config_clone, app_handle_clone));
//                         }
//                         Err(e) => {
//                              log::error!("TCP accept error: {}", e);
//                              // 可以选择是否在这里break
//                         }
//                     }
//                 }
//                 _ = &mut abort_rx => {
//                     log::info!("TCP server received stop signal");
//                     break;
//                 }
//             }
//         }
//     });

//     Ok(super::SpeedTestHandle {
//         abort_handle: abort_tx,
//     })
// }
