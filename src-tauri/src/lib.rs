#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod desktop_specific {
    pub mod analyze_thread;
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

// mod net_tool;

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
    let build = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init());
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
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        build
            .invoke_handler(tauri::generate_handler![get_platform,])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
