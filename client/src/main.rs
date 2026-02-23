use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use stun::get_ip_via_stun;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let peer = get_ip_via_stun().await;

    //tcp client
    let server_address = SocketAddr::from_str("127.0.0.1:8080")?;

    let mut tcp_stream = TcpStream::connect(server_address)?;

    //Sent Peer struct to server side
    // tcp_stream.write(peer)?;

    let message = "Hello from client";
    tcp_stream.write(message.as_bytes())?;

    Ok(())
}
