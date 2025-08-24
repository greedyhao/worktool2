// src-tauri/src/commands.rs

use crate::speed_test::{SpeedTestConfig, self};
use crate::state::AppState;
use tauri::{State, AppHandle, Manager};
use std::sync::Arc;

#[tauri::command]
pub async fn start_speed_test(
    config: SpeedTestConfig,
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut handle_guard = state.speed_test_handle.lock().await;
    if handle_guard.is_some() {
        return Err("Speed test is already running".into());
    }

    match speed_test::start_speed_test(config, app_handle.clone()).await {
        Ok(handle) => {
            *handle_guard = Some(handle);
            Ok(())
        }
        Err(e) => Err(format!("Failed to start speed test: {}", e)),
    }
}

#[tauri::command]
pub async fn stop_speed_test(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut handle_guard = state.speed_test_handle.lock().await;
    if let Some(handle) = handle_guard.take() {
        if let Err(e) = handle.abort_handle.send(()) {
            log::warn!("Failed to send abort signal: {:?}", e);
            // 即使发送失败，也认为停止了
        }
        log::info!("Speed test stop signal sent");
        Ok(())
    } else {
        Err("No speed test is currently running".into())
    }
}
