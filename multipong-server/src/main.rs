use std::net::UdpSocket;
use std::time::Instant;

fn main() {
    let socket = UdpSocket::bind("[::]:3123").expect("couldn't bind to address");
    loop {
        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let now = Instant::now();
                let string = format!("received {} bytes from {}", size, addr);
                println!("{}", string);
                let result = socket.send_to(string.as_bytes(), addr);
                if let Err(e) = result {
                    print!("{}", e);
                }
                println!("Elapsed: {}us", now.elapsed().as_micros());
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
