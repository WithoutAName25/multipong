use std::error::Error;
use std::net::UdpSocket;
use std::time::Instant;

fn send_test_msg(socket: &UdpSocket) -> Result<(), Box<dyn Error>> {
    socket.send_to("Hello".as_bytes(), "localhost:3123")?;
    Ok(())
}

fn receive_msg(socket: &UdpSocket) -> Result<String, Box<dyn Error>> {
    let mut buf = [0u8; 1024];
    let size = socket.recv(&mut buf)?;
    let msg = String::from_utf8(buf[0..size].to_owned())?;
    Ok(msg)
}

fn main() {
    let socket = &UdpSocket::bind("[::]:0").expect("couldn't bind to address");

    let now = Instant::now();
    send_test_msg(socket).expect("Failed to send");
    println!("Received answer after {}ms: {}", now.elapsed().as_millis(), receive_msg(socket).expect("Failed to receive"));
}