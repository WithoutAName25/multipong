use multipong_common::packages::{LoginRequest, Packet};
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("[::]:3123").expect("couldn't bind to address");
    loop {
        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                if size >= 1 && buf[0] == LoginRequest::packet_id() {
                    let request = LoginRequest::read_data(&buf[1..size]).unwrap();
                    println!("{}: {:?}", addr, request);
                } else {
                    println!("{}: {:?}", addr, &buf[..size]);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
