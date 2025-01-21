use serde::Serialize;

use crate::utils::process_ascii_lines_from_file;

#[derive(Serialize, Default, Debug, Clone)]
pub struct CPURegs {
    regs: [String; 32], // 32 个寄存器
    header: String,     // 寄存器组的标题
}

#[tauri::command]
pub fn process_exception_log(file_path: &str) -> Result<CPURegs, String> {
    let start_flag1 = "ERR:";
    let start_flag2 = "EPC:";
    let start_flag3 = "WDT_RST:";

    let empty_str = "0xXXXXXXXX";
    let mut regs = CPURegs::default();
    // let mut reg_vec = Vec::new();

    let mut index = 0;
    let mut state = 0; // 1: epc, 2: wdt
    let ret = process_ascii_lines_from_file(file_path, |line| {
        // println!("line: {}, state:{}", line, state);
        match state {
            1 => {
                for l in line.split(' ') {
                    if l.len() == 0 {
                        continue;
                    }
                    if let Ok(reg) = u32::from_str_radix(l, 16) {
                        regs.regs[index] = format!("{:#010X}", reg);
                    } else {
                        state = 3;
                    }

                    index += 1;
                }
                if index >= 32 {
                    // state = 3;
                    // reg_vec.push(regs.clone());
                    return false;
                }
            }
            2 => {
                for l in line.split(' ') {
                    match index {
                        0 => {
                            regs.regs[index] = empty_str.to_string();
                            index += 1;
                        }
                        2 => {
                            while index < 4 {
                                regs.regs[index] = empty_str.to_string();
                                index += 1;
                            }
                        }
                        18 => {
                            while index < 28 {
                                regs.regs[index] = empty_str.to_string();
                                index += 1;
                            }
                        }
                        _ => {}
                    }
                    if l.len() == 0 {
                        continue;
                    }

                    if let Ok(reg) = u32::from_str_radix(l, 16) {
                        regs.regs[index] = format!("{:#010X}", reg);
                    } else {
                        state = 3;
                    }

                    index += 1;
                }
                if index >= 19 {
                    // state = 3;
                    // reg_vec.push(regs.clone());
                    return false;
                }
            }
            _ => {}
        }

        if line.contains(start_flag1) && line.contains(start_flag2) {
            regs.header = line.to_string();
            state = 1;
            index = 0;
            // println!("EPC");
        }
        if line.contains(start_flag3) {
            regs.header = line.to_string();
            state = 2;
            index = 0;
            // println!("WDT");
        }
        true
    });

    match ret {
        Ok(_) => Ok(regs),
        Err(e) => Err(e.to_string()),
    }
}
