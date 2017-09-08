#![feature(tcpstream_connect_timeout)]
extern crate ipnetwork;
extern crate local_ip;

use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use ipnetwork::Ipv4Network;
use std::net::TcpStream;
use std::time::Duration;
use std::io::prelude::*;

fn main() {
    if let Some(ip) = local_ip::get() {
        let net: Ipv4Network = format!("{}/24", ip).parse().unwrap();
        for host in net.iter() {
            if connect(host) {
                println!("{}", host);
            }
        }
    }
    println!("Press Enter to close window");
    let mut stdin = std::io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn connect(host: Ipv4Addr) -> bool {
    let socket = SocketAddr::new(IpAddr::V4(host), 9100);
    match TcpStream::connect_timeout(&socket, Duration::new(0,5000000)) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
