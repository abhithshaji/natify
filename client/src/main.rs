use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::str::FromStr;
use stun::get_ip_via_stun;
use types::Peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let self_details = get_ip_via_stun().await;

    //tcp client
    let server_address = SocketAddr::from_str("127.0.0.1:8080")?;

    let mut tcp_stream = TcpStream::connect(server_address)?;

    let serialized_self_details = serde_json::to_string(&self_details)?;

    //Sent Peer struct (Self details) to server side
    tcp_stream.write(serialized_self_details.as_bytes())?;

    //receive the other peer's details
    let mut buf: [u8; 1024] = [0; 1024];
    tcp_stream.read(&mut buf)?;
    let peer_details: Peer = serde_json::from_str(str::from_utf8(&buf)?.trim_end_matches("\0"))?;

    // create a udp socket
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", self_details.port_no))?;

    // send data to peer
    socket.send_to(
        "hole punch request".as_bytes(),
        format!("127.0.0.1:{}", peer_details.port_no),
    )?;
    println!("hole punch request sent");

    // receive data from peer
    let mut received_message_buf: [u8; 1024] = [0; 1024];
    socket.recv_from(&mut received_message_buf)?;

    println!(
        "Recieved Message : {:?}",
        str::from_utf8(&received_message_buf)?.trim_end_matches("\0")
    );

    Ok(())
}
