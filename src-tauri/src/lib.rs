#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod utils;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod desktop_specific {
    pub mod analyze_thread;
    pub mod audio_converter;
    mod audio_decoder;
    pub mod exception_log;
    pub mod hci_log;
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use desktop_specific::*;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use analyze_thread::{analyze_thread_plot, analyze_thread_preprocess, generate_plot};
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use exception_log::process_exception_log;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use hci_log::parse_hci_log;
// #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
// use audio_converter::convert_audio;

mod net_tool;
mod wav_decoder;
use net_tool::{nettool_init, nettool_start_test, nettool_stop_test};

use tauri::Manager;

mod commands;
mod speed_test;
mod state;

use state::AppState;

use env_logger::Env;

#[tauri::command]
fn get_platform() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else {
        "unknown".to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let build = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .manage(AppState::new());
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        build
            .plugin(tauri_plugin_process::init())
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![
                get_platform,
                process_exception_log,
                generate_plot,
                analyze_thread_preprocess,
                analyze_thread_plot,
                parse_hci_log,
                nettool_start_test,
                nettool_stop_test,
                commands::start_speed_test,
                commands::stop_speed_test,
                // convert_audio
            ])
            .setup(|app| {
                app.manage(nettool_init());
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        build
            .invoke_handler(tauri::generate_handler![
                get_platform,
                nettool_start_test,
                nettool_stop_test,
                commands::start_speed_test,
                commands::stop_speed_test,
            ])
            .setup(|app| {
                app.manage(nettool_init());
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
