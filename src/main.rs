use std::net::UdpSocket;

const SERVER_ADRESS: &str = "127.0.0.1:34254";

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(SERVER_ADRESS)?;
    let mut buf = [0; 10];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
    socket.send_to(&buf[..number_of_bytes], src_addr)?;
    Ok(())
}
