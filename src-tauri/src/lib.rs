use serde::Serialize;
use tauri::command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct CPURegs {
    regs: [String; 32], // 32 个寄存器
    header: String,     // 寄存器组的标题
}

#[command]
fn process_exception_log(file_path: String) -> Result<CPURegs, String> {
    // 模拟处理异常日志文件
    if file_path.is_empty() {
        return Err("文件地址不能为空".to_string());
    }

    // 模拟返回的寄存器组数据
    let result = CPURegs {
        regs: [
            "0x00000000".to_string(),
            "0x00000001".to_string(),
            "0x00000002".to_string(),
            // 填充剩余的寄存器值
            "0x00000003".to_string(),
            "0x00000004".to_string(),
            "0x00000005".to_string(),
            "0x00000006".to_string(),
            "0x00000007".to_string(),
            "0x00000008".to_string(),
            "0x00000009".to_string(),
            "0x0000000A".to_string(),
            "0x0000000B".to_string(),
            "0x0000000C".to_string(),
            "0x0000000D".to_string(),
            "0x0000000E".to_string(),
            "0x0000000F".to_string(),
            "0x00000010".to_string(),
            "0x00000011".to_string(),
            "0x00000012".to_string(),
            "0x00000013".to_string(),
            "0x00000014".to_string(),
            "0x00000015".to_string(),
            "0x00000016".to_string(),
            "0x00000017".to_string(),
            "0x00000018".to_string(),
            "0x00000019".to_string(),
            "0x0000001A".to_string(),
            "0x0000001B".to_string(),
            "0x0000001C".to_string(),
            "0x0000001D".to_string(),
            "0x0000001E".to_string(),
            "0x0000001F".to_string(),
        ],
        header: "CPU 寄存器组".to_string(),
    };

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, process_exception_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
