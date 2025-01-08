use plotly::common::Mode;
use plotly::{Plot, Scatter};

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

use regex::Regex;

#[tauri::command]
pub fn generate_plot() -> String {
    let trace = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).mode(Mode::LinesMarkers);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    return plot.to_inline_html(None);
}

/// 分析主函数
#[derive(Debug)]
struct LogMessage {
    timestamp: f64,
    content: String,
}

fn process_logic_data(reader: impl BufRead) -> Result<Vec<LogMessage>, Box<dyn Error>> {
    let mut messages = Vec::new();
    let mut current_message = String::with_capacity(100); // 预分配内存
    let mut start_time: Option<f64> = None;

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(reader);

    // 获取并缓存列索引
    let headers = csv_reader.headers()?;
    let time_idx = headers
        .iter()
        .position(|h| h == "Time [s]")
        .ok_or("Missing 'Time [s]' column")?;
    let mosi_idx = headers
        .iter()
        .position(|h| h == "MOSI")
        .ok_or("Missing 'MOSI' column")?;

    // 使用 into_records 避免克隆
    for result in csv_reader.into_records() {
        let record = result?;

        let time: f64 = record.get(time_idx).ok_or("Missing time field")?.parse()?;

        let char = record.get(mosi_idx).ok_or("Missing MOSI field")?;

        if start_time.is_none() {
            start_time = Some(time);
        }

        match char {
            "NUL" => current_message.push(' '),
            "LF " => {
                if !current_message.is_empty() {
                    let message = current_message.trim().to_string();
                    if message.contains(':') {
                        if let Some(timestamp) = start_time {
                            messages.push(LogMessage {
                                timestamp,
                                content: message,
                            });
                        }
                        start_time = None;
                    }
                }
                current_message.clear();
            }
            c => current_message.push_str(c),
        }
    }

    // let len = messages.len();

    Ok(messages)
}

#[inline]
fn validate_message_type(message: &str, map: &HashSet<String>) -> bool {
    if let Some(msg_type) = message.split(':').next() {
        map.contains(msg_type)
    } else {
        false
    }
}

fn write_output(messages: &[LogMessage], output_file: &str) -> io::Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    for message in messages {
        writeln!(writer, "[{:.6}]{}", message.timestamp, message.content)?;
    }

    writer.flush()?;
    Ok(())
}

#[tauri::command]
pub fn analyze_thread_preprocess(
    input_file: &str,
    output_file: &str,
) -> Result<Vec<String>, Box<String>> {
    // let valid_types: HashSet<String> = types.split(',').map(|s| s.trim().to_string()).collect();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::with_capacity(128 * 1024, file);

    let messages = process_logic_data(reader).unwrap();

    let mut types = HashSet::new();
    // 遍历 messages，提取 content 并插入 HashSet
    for message in &messages {
        types.insert(message.content.clone());
    }
    let types = types.into_iter().collect::<Vec<String>>();

    let valid_messages: Vec<_> = messages.into_iter().collect();
    write_output(&valid_messages, output_file).unwrap();
    Ok(types)
}
