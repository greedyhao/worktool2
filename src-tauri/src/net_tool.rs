use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;
use std::time::{Duration, Instant};

const BUFFER_SIZE: usize = 1024;

use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter, Manager, Runtime, State};

// 状态结构体
pub struct NetToolState {
    running: AtomicBool,
}

// 初始化方法
pub fn nettool_init() -> NetToolState {
    NetToolState {
        running: AtomicBool::new(false),
    }
}

#[derive(Deserialize)]
pub struct NetToolConfig {
    mode: String,
    role: String,
    address: String,
    port: u16,
    interval: u64,
    duration: Option<u64>,
}

#[tauri::command]
pub fn nettool_start_test<R: Runtime>(
    app: AppHandle<R>,
    config: NetToolConfig,
) -> Result<(), String> {
    let state = app.state::<NetToolState>();

    // 使用原子操作检查是否已在运行
    if state
        .running
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
        .is_err()
    {
        return Err("Test is already running".into());
    }

    let mode = &config.mode;
    let role = &config.role;
    match (mode.as_str(), role.as_str()) {
        ("tcp", "client") => {
            test_tcp_client(app, &config);
        }
        ("tcp", "server") => {
            test_tcp_server(app, &config);
        }
        ("udp", "client") => {
            test_udp_client(app, &config);
        }
        ("udp", "server") => {
            test_udp_server(app, &config);
        }
        _ => eprintln!(
            "Invalid mode or role. Use 'tcp' or 'udp' for mode, and 'client' or 'server' for role."
        ),
    };

    Ok(())
}

#[tauri::command]
pub fn nettool_stop_test<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let state = app.state::<NetToolState>();
    state.running.store(false, Ordering::SeqCst);
    println!("stop test");
    Ok(())
}

fn test_thread<R: Runtime, F: FnMut(AppHandle<R>) -> bool + Send + 'static>(
    app: AppHandle<R>,
    config: &NetToolConfig,
    f: F,
) {
    let duration = config.duration;

    // 生成独立线程
    thread::spawn(move || {
        let state: State<'_, NetToolState> = app.state::<NetToolState>();
        let start_time = Instant::now();
        let mut f = f;

        println!("starting test: {:?} {:?}", start_time, duration);

        loop {
            if stop_check(&state, duration, start_time) {
                break;
            }

            f(app.clone());
        }

        println!("background thread exited");
    });
}

fn stop_check(state: &State<'_, NetToolState>, duration: Option<u64>, start_time: Instant) -> bool {
    // 检查停止条件（原子读取）
    if !state.running.load(Ordering::SeqCst) {
        return true;
    }

    // 检查持续时间限制
    if let Some(duration) = duration {
        if start_time.elapsed() > Duration::from_secs(duration) {
            state.running.store(false, Ordering::SeqCst);
            return true;
        }
    }
    false
}

fn test_tcp_client<R: Runtime>(app: AppHandle<R>, config: &NetToolConfig) {
    let address = &config.address;
    let server_addr = format!("{}:{}", address, config.port);
    let interval = config.interval;

    println!(
        "Testing TCP client performance with server: {}",
        server_addr
    );

    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");
    let buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    test_thread(app, config, move |app| {
        match stream.write(&buffer) {
            Ok(bytes_written) => {
                total_bytes += bytes_written;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    app.emit("rate-update", rate).unwrap();
                    // println!("Sent: {} KB, Rate: {:.2} KB/s", total_bytes / 1024, rate);
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError writing to TCP stream: {}", e);
                return false;
            }
        }
        return true;
    });
}

fn test_tcp_server<R: Runtime>(app: AppHandle<R>, config: &NetToolConfig) {
    let server_addr = format!("127.0.0.1:{}", config.port);
    let interval = config.interval;

    println!("Testing TCP server performance on: {}", server_addr);

    let listener = TcpListener::bind(server_addr).expect("Failed to bind TCP listener");
    let (mut stream, addr) = listener.accept().expect("Failed to accept connection");
    println!("Client connected: {}", addr);

    let mut buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    test_thread(app, config, move |app| {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("\nClient disconnected.");
                    return false;
                }
                total_bytes += bytes_read;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    app.emit("rate-update", rate).unwrap();
                    // println!(
                    //     "Received: {} KB, Rate: {:.2} KB/s",
                    //     total_bytes / 1024,
                    //     rate
                    // );
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError reading from TCP stream: {}", e);
                return false;
            }
        }
        return true;
    });
}

fn test_udp_client<R: Runtime>(app: AppHandle<R>, config: &NetToolConfig) {
    let address = &config.address;
    let server_addr = format!("{}:{}", address, config.port);
    let interval = config.interval;
    println!(
        "Testing UDP client performance with server: {}",
        server_addr
    );

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");
    let server_addr: std::net::SocketAddr = server_addr.parse().expect("Invalid server address");
    let buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    test_thread(app, config, move |app| {
        match socket.send_to(&buffer, &server_addr) {
            Ok(bytes_sent) => {
                total_bytes += bytes_sent;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    app.emit("rate-update", rate).unwrap();
                    // println!("Sent: {} KB, Rate: {:.2} KB/s", total_bytes / 1024, rate);
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError sending UDP packet: {}", e);
                return false;
            }
        }
        return true;
    });
}

fn test_udp_server<R: Runtime>(app: AppHandle<R>, config: &NetToolConfig) {
    let server_addr = format!("127.0.0.1:{}", config.port);
    let interval = config.interval;
    println!("Testing UDP server performance on: {}", server_addr);

    let socket = UdpSocket::bind(server_addr).expect("Failed to bind UDP socket");
    let mut buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    test_thread(app, config, move |app| {
        match socket.recv_from(&mut buffer) {
            Ok((bytes_read, _addr)) => {
                total_bytes += bytes_read;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    app.emit("rate-update", rate).unwrap();
                    // println!(
                    //     "Received: {} KB, Rate: {:.2} KB/s",
                    //     total_bytes / 1024,
                    //     rate
                    // );
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError receiving UDP packet: {}", e);
                return false;
            }
        }
        return true;
    });
}
