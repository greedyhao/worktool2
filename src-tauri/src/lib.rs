mod analyze_thread;
use analyze_thread::generate_plot;
use analyze_thread::{analyze_thread_plot, analyze_thread_preprocess};

mod exception_log;
use exception_log::process_exception_log;

mod hci_log;
use hci_log::parse_hci_log;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            process_exception_log,
            generate_plot,
            analyze_thread_preprocess,
            analyze_thread_plot,
            parse_hci_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
