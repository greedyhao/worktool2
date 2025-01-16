use std::net::{TcpStream, TcpListener, UdpSocket};
use std::time::{Instant, Duration};
use std::io::{self, Write, Read};

const BUFFER_SIZE: usize = 1024;

fn net_tool_main() {
    // let matches = Command::new("Network Performance Tester")
    //     .version("1.0")
    //     .author("Your Name <your.email@example.com>")
    //     .about("Tests network performance using TCP or UDP")
    //     .arg(Arg::new("mode")
    //         .short('m')
    //         .long("mode")
    //         .value_name("MODE")
    //         .help("Sets the test mode (tcp or udp)")
    //         .required(true))
    //     .arg(Arg::new("role")
    //         .short('r')
    //         .long("role")
    //         .value_name("ROLE")
    //         .help("Sets the role (client or server)")
    //         .required(true))
    //     .arg(Arg::new("address")
    //         .short('a')
    //         .long("address")
    //         .value_name("ADDRESS")
    //         .help("Sets the server address (required for client)"))
    //     .arg(Arg::new("port")
    //         .short('p')
    //         .long("port")
    //         .value_name("PORT")
    //         .help("Sets the server port")
    //         .required(true))
    //     .arg(Arg::new("interval")
    //         .short('i')
    //         .long("interval")
    //         .value_name("INTERVAL")
    //         .help("Sets the interval (in seconds) for printing rate (default: 5)")
    //         .default_value("5"))
    //     .get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();
    let role = matches.get_one::<String>("role").unwrap();
    let port = matches.get_one::<String>("port").unwrap();
    let interval: u64 = matches.get_one::<String>("interval")
        .unwrap()
        .parse()
        .expect("Interval must be a number");

    match (mode.as_str(), role.as_str()) {
        ("tcp", "client") => {
            let address = matches.get_one::<String>("address")
                .expect("Address is required for client mode");
            let full_addr = format!("{}:{}", address, port);
            test_tcp_client(&full_addr, interval);
        }
        ("tcp", "server") => {
            let full_addr = format!("0.0.0.0:{}", port);
            test_tcp_server(&full_addr, interval);
        }
        ("udp", "client") => {
            let address = matches.get_one::<String>("address")
                .expect("Address is required for client mode");
            let full_addr = format!("{}:{}", address, port);
            test_udp_client(&full_addr, interval);
        }
        ("udp", "server") => {
            let full_addr = format!("0.0.0.0:{}", port);
            test_udp_server(&full_addr, interval);
        }
        _ => eprintln!("Invalid mode or role. Use 'tcp' or 'udp' for mode, and 'client' or 'server' for role."),
    }
}

fn test_tcp_client(server_addr: &str, interval: u64) {
    println!("Testing TCP client performance with server: {}", server_addr);

    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");
    let mut buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    loop {
        match stream.write(&buffer) {
            Ok(bytes_written) => {
                total_bytes += bytes_written;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    println!("Sent: {} KB, Rate: {:.2} KB/s", total_bytes / 1024, rate);
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError writing to TCP stream: {}", e);
                break;
            }
        }
    }
}

fn test_tcp_server(server_addr: &str, interval: u64) {
    println!("Testing TCP server performance on: {}", server_addr);

    let listener = TcpListener::bind(server_addr).expect("Failed to bind TCP listener");
    let (mut stream, addr) = listener.accept().expect("Failed to accept connection");
    println!("Client connected: {}", addr);

    let mut buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("\nClient disconnected.");
                    break;
                }
                total_bytes += bytes_read;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    println!("Received: {} KB, Rate: {:.2} KB/s", total_bytes / 1024, rate);
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError reading from TCP stream: {}", e);
                break;
            }
        }
    }
}

fn test_udp_client(server_addr: &str, interval: u64) {
    println!("Testing UDP client performance with server: {}", server_addr);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");
    let server_addr: std::net::SocketAddr = server_addr.parse().expect("Invalid server address");
    let buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    loop {
        match socket.send_to(&buffer, &server_addr) {
            Ok(bytes_sent) => {
                total_bytes += bytes_sent;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    println!("Sent: {} KB, Rate: {:.2} KB/s", total_bytes / 1024, rate);
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError sending UDP packet: {}", e);
                break;
            }
        }
    }
}

fn test_udp_server(server_addr: &str, interval: u64) {
    println!("Testing UDP server performance on: {}", server_addr);

    let socket = UdpSocket::bind(server_addr).expect("Failed to bind UDP socket");
    let mut buffer = [0u8; BUFFER_SIZE];
    let start_time = Instant::now();
    let mut total_bytes = 0;
    let mut last_print_time = Instant::now();

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((bytes_read, addr)) => {
                total_bytes += bytes_read;
                let elapsed = start_time.elapsed().as_secs_f64();
                if last_print_time.elapsed() >= Duration::from_secs(interval) {
                    let rate = (total_bytes as f64) / elapsed / 1024.0; // KB/s
                    println!("Received: {} KB, Rate: {:.2} KB/s", total_bytes / 1024, rate);
                    last_print_time = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("\nError receiving UDP packet: {}", e);
                break;
            }
        }
    }
}