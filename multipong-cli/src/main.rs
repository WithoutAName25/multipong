use multipong_common::packets::Packet;
use multipong_common::packets::Packet::LoginRequest;
use multipong_common::serialization::Serializable;
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
        .serialize(&mut buf)
        .expect("couldn't serialize packet");

    socket
        .send_to(&buf[..size], server)
        .expect("couldn't send data");

    let (size, addr) = socket.recv_from(&mut buf).expect("couldn't receive data");
    let (_, response) = Packet::deserialize(&buf[..size]).expect("couldn't deserialize packet");
    println!("Received packet from {}: {:?}", addr, response);
}
