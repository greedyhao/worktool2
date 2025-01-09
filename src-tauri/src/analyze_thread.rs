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
            "NUL" | "(SP)" => current_message.push(' '),
            "LF " => {
                if !current_message.is_empty() {
                    let message = current_message.trim().to_string();
                    if validate_message_content(&message) {
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
            c => {
                if c.len() == 1 {
                    current_message.push_str(c)
                }
            }
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

fn validate_message_content(content: &str) -> bool {
    content.chars().filter(|c| *c == ':').count() == 1
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
        if let Some((t, _)) = message.content.split_once(':') {
            types.insert(t.to_string());
        }
    }

    if types.len() > 20 {
        types.retain(|s| {
            !s.contains(&[
                '#', '!', '$', '%', '&', '*', '(', ')', '=', '+', '-', '_', '[', ']', '{', '}',
                ';', ':', '<', '>', '/', '\\', '?', '@', '`', ' ', '~',
            ]) && !s.starts_with(&['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'])
                && (s.len() > 0)
        });
    }

    let valid_messages: Vec<_> = messages
        .into_iter()
        .filter(|msg| validate_message_type(&msg.content, &types))
        .collect();
    let types = types.into_iter().collect::<Vec<String>>();

    write_output(&valid_messages, output_file).unwrap();
    Ok(types)
}

use plotly::common::Mode;
use plotly::layout::{GridPattern, LayoutGrid, RowOrder};
use plotly::{Layout, Pie, Plot, Scatter};

/// 绘制CPU使用率
#[derive(Debug)]
struct ThreadSwitch {
    timestamp: f64,
    #[allow(dead_code)]
    from_thread: String,
    to_thread: String,
    #[allow(dead_code)]
    ra: String,
}

fn parse_thread_switch_log(filename: &str, choiced: &str) -> io::Result<Vec<ThreadSwitch>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let re = format!(r"\[([\d.]+)\]{}:(\w+)\s+(\w+)\s+([\da-fA-F]+)", choiced);
    let pattern = Regex::new(&re).unwrap();
    let mut switches = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(captures) = pattern.captures(&line) {
            let timestamp = captures[1].parse::<f64>().unwrap();
            let from_thread = captures[2].to_string();
            let to_thread = captures[3].to_string();
            let ra = captures[4].to_string();
            switches.push(ThreadSwitch {
                timestamp,
                from_thread,
                to_thread,
                ra,
            });
        }
    }

    Ok(switches)
}

fn analyze_cpu_usage(
    switches: &[ThreadSwitch],
) -> (HashMap<String, f64>, HashMap<String, Vec<(f64, f64, f64)>>) {
    let mut thread_times = HashMap::new();
    let mut thread_intervals = HashMap::new();

    for i in 0..switches.len() - 1 {
        let current = &switches[i];
        let next_switch = &switches[i + 1];
        let duration = next_switch.timestamp - current.timestamp;
        let thread = &current.to_thread;

        *thread_times.entry(thread.clone()).or_insert(0.0) += duration;
        thread_intervals
            .entry(thread.clone())
            .or_insert_with(Vec::new)
            .push((current.timestamp, next_switch.timestamp, duration));
    }

    (thread_times, thread_intervals)
}

fn plot_cpu_usage(
    thread_times: &HashMap<String, f64>,
    thread_intervals: &HashMap<String, Vec<(f64, f64, f64)>>,
    window_size: f64,
) -> String {
    let mut plot = Plot::new();

    // 创建饼图
    let threads: Vec<String> = thread_times.keys().cloned().collect();
    let times: Vec<f64> = thread_times.values().cloned().collect();

    let pie = Pie::new(times.clone())
        .labels(threads.clone())
        .name("CPU Usage");

    plot.add_trace(pie);

    // 创建折线图
    let total_time = thread_intervals
        .values()
        .flat_map(|intervals| intervals.iter().map(|interval| interval.1))
        .fold(0.0, |acc, x| if x > acc { x } else { acc });

    let time_points: Vec<f64> = (0..=(total_time / window_size) as usize)
        .map(|i| i as f64 * window_size)
        .collect();

    for (thread, intervals) in thread_intervals {
        let usage: Vec<f64> = time_points
            .iter()
            .map(|&t| {
                intervals
                    .iter()
                    .filter(|interval| interval.0 < t + window_size && interval.1 > t)
                    .map(|interval| {
                        (interval.1.min(t + window_size) - interval.0.max(t)) / window_size * 100.0
                    })
                    .sum()
            })
            .collect();

        let trace = Scatter::new(time_points.clone(), usage)
            .name(thread)
            .mode(Mode::Lines)
            .x_axis("x2") // 指定线图使用第二个 x 轴
            .y_axis("y2"); // 指定线图使用第二个 y 轴;
        plot.add_trace(trace);
    }

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(2)
            .columns(1)
            .pattern(GridPattern::Independent)
            .row_order(RowOrder::TopToBottom),
    );
    plot.set_layout(layout);

    // 返回 HTML 字符串
    plot.to_inline_html(None)
}

#[tauri::command]
pub fn analyze_thread_plot(choiced: &str, input_file: &str) -> Result<String, Box<String>> {
    let switches = parse_thread_switch_log(input_file, choiced);
    match switches {
        Ok(switches) => {
            let (thread_times, thread_intervals) = analyze_cpu_usage(&switches);
            return Ok(plot_cpu_usage(&thread_times, &thread_intervals, 0.1));
        }
        Err(err) => {
            return Err(Box::new(err.to_string()));
        }
    }
}
