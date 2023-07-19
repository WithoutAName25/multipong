use multipong_common::packages::{LoginRequest, Packet};
use std::net::{ToSocketAddrs, UdpSocket};

fn main() {
    let socket = &UdpSocket::bind("[::]:0").expect("couldn't bind to address");
    let server = "localhost:3123"
        .to_socket_addrs()
        .expect("couldn't resolve address")
        .next()
        .unwrap();

    let mut buf = [0u8; 1024];
    let request = LoginRequest {
        username: "test".to_string(),
    };
    let size = request
        .write_packet(&mut buf)
        .expect("buffer is big enough");

    socket
        .send_to(&buf[..size], server)
        .expect("couldn't send data");
}
