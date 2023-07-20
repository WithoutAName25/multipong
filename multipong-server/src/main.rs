use bimap::BiHashMap;
use multipong_common::packets::Packet;
use multipong_common::packets::Packet::LoginRequest;
use multipong_common::serialization::Serializable;
use std::net::{SocketAddr, UdpSocket};

fn main() {
    let socket = UdpSocket::bind("[::]:3123").expect("couldn't bind to address");
    let mut users = BiHashMap::<SocketAddr, String>::new();
    loop {
        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let (_, packet) =
                    Packet::deserialize(&buf[..size]).expect("couldn't deserialize packet");
                println!("Received packet from {}: {:?}", addr, packet);
                match packet {
                    LoginRequest { username } => {
                        println!("{} requested username '{}'", addr, username);
                        let success = if users.get_by_left(&addr) == Some(&username) {
                            println!("{} already logged in as '{}'", addr, username);
                            true
                        } else if users.contains_right(&username) {
                            println!(
                                "'{}' already used by {}",
                                username,
                                users.get_by_right(&username).unwrap()
                            );
                            false
                        } else {
                            println!("{} logged in as '{}'", addr, username);
                            users.insert(addr, username);
                            true
                        };
                        let response = Packet::LoginResponse { success };
                        let size = response
                            .serialize(&mut buf)
                            .expect("couldn't serialize packet");
                        socket
                            .send_to(&buf[..size], addr)
                            .expect("couldn't send data");
                    }
                    _ => {
                        println!("Received unknown packet from {}: {:?}", addr, packet);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
