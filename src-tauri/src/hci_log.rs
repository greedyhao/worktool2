use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Error, Write};
use std::time::UNIX_EPOCH;

#[tauri::command]
pub fn parse_hci_log(file_path: &str) -> Result<(), String> {
    match parse_hci_log_do(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_hci_log_do(file_path: &str) -> io::Result<()> {
    // 打开 HCI Log 文件
    let hci_log_file = File::open(file_path)?;
    let reader = io::BufReader::new(hci_log_file);

    let cfa_file = format!("{}.cfa", file_path);
    // 创建 BTSnoop 文件
    let mut btsnoop_file = BufWriter::new(File::create(&cfa_file)?);

    // 写入 BTSnoop 文件头
    btsnoop_file.write_all(b"btsnoop\0")?; // 标识符
    btsnoop_file.write_all(&1u32.to_be_bytes())?; // 版本号
    btsnoop_file.write_all(&1002u32.to_be_bytes())?; // 数据链路类型 (HCI UART)

    // 获取文件元数据
    let metadata = fs::metadata(file_path)?;
    let modified_time = metadata.modified()?;
    let modified_time = modified_time.duration_since(UNIX_EPOCH).unwrap().as_secs();

    // 解析 HCI Log 并写入 BTSnoop 数据包记录
    for line in reader.lines() {
        let line = line?;

        // 过滤无效行
        if !is_valid_line(&line) {
            continue;
        }

        // 解析行内容
        let parts: Vec<&str> = line.split_whitespace().collect();

        // 解析时间戳
        let timestamp_str = parts[0]; // 例如 "[00:00:02.740]"
        let timestamp = parse_timestamp(modified_time, timestamp_str)?;

        let packet_type = parts[1]; // 包类型 (CMD, EVT, ACL)
        let direction = parts[2]; // 方向 (=>, <=)
        let hci_data: Vec<u8> = parts[3..]
            .iter()
            .filter_map(|x| u8::from_str_radix(x, 16).ok())
            .collect(); // HCI 数据内容

        // 添加 HCI UART 头
        let h4_header = match packet_type {
            "CMD" => 0x01, // 命令
            "ACL" => 0x02, // ACL 数据
            "EVT" => 0x04, // 事件
            _ => continue, // 忽略未知类型
        };

        let mut packet_data = Vec::new();
        packet_data.push(h4_header); // 添加 HCI UART 头
        packet_data.extend(hci_data); // 添加 HCI 数据

        // 计算 BTSnoop 标志
        let flags: u32 = match (packet_type, direction) {
            ("CMD", "=>") => 0x02, // 发送命令 (bit1=1, bit0=0)
            ("EVT", "<=") => 0x03, // 接收事件 (bit1=1, bit0=1)
            ("ACL", "=>") => 0x00, // 发送 ACL 数据 (bit1=0, bit0=0)
            ("ACL", "<=") => 0x01, // 接收 ACL 数据 (bit1=0, bit0=1)
            _ => return Err(Error::other("Unknown packet type or direction")), // 忽略未知类型或方向
        };

        // 写入 BTSnoop 数据包记录
        btsnoop_file.write_all(&(packet_data.len() as u32).to_be_bytes())?; // 原始长度
        btsnoop_file.write_all(&(packet_data.len() as u32).to_be_bytes())?; // 包含长度
        btsnoop_file.write_all(&flags.to_be_bytes())?; // 标志
        btsnoop_file.write_all(&0_u32.to_be_bytes())?;
        btsnoop_file.write_all(&timestamp.to_be_bytes())?; // 累计微秒数
        btsnoop_file.write_all(&packet_data)?; // 数据包内容
    }

    Ok(())
}

// 检查行是否有效
fn is_valid_line(line: &str) -> bool {
    // 行必须包含时间戳、包类型、方向和至少一个数据字节
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        return false;
    }

    // 检查时间戳格式
    if !parts[0].starts_with('[') || !parts[0].ends_with(']') {
        return false;
    }

    // 检查包类型
    let packet_type = parts[1];
    if packet_type != "CMD" && packet_type != "ACL" && packet_type != "EVT" {
        return false;
    }

    // 检查方向
    let direction = parts[2];
    if direction != "=>" && direction != "<=" {
        return false;
    }

    // 检查数据部分是否为有效的十六进制
    parts[3..].iter().all(|x| u8::from_str_radix(x, 16).is_ok())
}

// 解析时间戳
fn parse_timestamp(modified: u64, timestamp_str: &str) -> io::Result<u64> {
    // 去掉时间戳的方括号
    let timestamp_str = timestamp_str.trim_start_matches('[').trim_end_matches(']');

    // 分割为秒和微秒部分
    let parts: Vec<&str> = timestamp_str.split('.').collect();
    if parts.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid timestamp format",
        ));
    }

    // 解析秒部分
    let time_parts: Vec<&str> = parts[0].split(':').collect();
    if time_parts.len() != 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid timestamp format",
        ));
    }

    let hours: u32 = time_parts[0]
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse hours"))?;
    let minutes: u32 = time_parts[1]
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse minutes"))?;
    let seconds: u32 = time_parts[2]
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse seconds"))?;

    // 解析微秒部分
    let microseconds: u32 = parts[1]
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse microseconds"))?;

    // 计算总秒数和微秒数
    let total_seconds = hours * 3600 + minutes * 60 + seconds;

    // let time_betw_0_and_2000_ad = 0x00E03AB44A676000;
    // let total = (modified + total_seconds as u64) * 1000000 + microseconds as u64 - 0x35D013B37E000
    //     + time_betw_0_and_2000_ad;
    let time_betw_0_and_1970_ad = 0x00DCDDB30F2F8000;
    let total =
        (modified + total_seconds as u64) * 1000000 + microseconds as u64 + time_betw_0_and_1970_ad;
    Ok(total)
}
