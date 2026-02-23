use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SocketAddr::from_str("127.0.0.1:8080")?;

    println!("Server Listening on {:?}", &server_address);
    let listener = TcpListener::bind(server_address).unwrap();

    for stream in listener.incoming() {
        let mut tcp_stream = stream?;
        // let peer_address = tcp_stream.peer_addr()?;

        let mut message: [u8; 1024] = [0; 1024];
        tcp_stream.read(&mut message)?;

        println!("{:?}", str::from_utf8(&message)?.trim_end_matches("\0"));
    }

    Ok(())
}
