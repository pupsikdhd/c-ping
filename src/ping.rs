
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use colored::Colorize;

pub fn ping(host: &str, port_opt: Option<u16>, timeout: Option<u16>) {
    let port = port_opt.unwrap_or(443);
    let addr_str = format!("{host}:{port}");

    let timeout = Duration::from_secs(timeout.unwrap_or(4) as u64);

    let addrs = match addr_str.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(..) => {
            println!("{} Unable to resolve name: {}", "FAIL".bright_red().bold(), addr_str.bright_black().bold());
            return;
        }
    };

    let mut connected = false;
    let mut last_err = None;

    for addr in addrs {
        let start = Instant::now();
        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(_) => {
                let ms = start.elapsed().as_millis();
                println!("{} {}ms → {} ({})",
                         "OK".bright_green().bold(),
                         ms.to_string().bright_white(),
                         addr_str.dimmed(),
                         addr);
                connected = true;
                break;
            }
            Err(e) => {
                last_err = Some(e);
            }
        }
    }

    if !connected {
        let err_msg = last_err.map_or("Unknown error".to_string(), |e| e.to_string());
        println!("{} {} → {}",
                 "FAIL".bright_red().bold(),
                 err_msg.red(),
                 addr_str.dimmed());
    }
}