use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use types::Peer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SocketAddr::from_str("127.0.0.1:8080")?;

    //list of peers
    let mut peer_list: Vec<Peer> = vec![];

    //tcp stream holding queue, for clients that sent data but have no peers at the moment
    let mut tcp_stream_list: Vec<TcpStream> = vec![];

    println!("Server Listening on {:?}", &server_address);
    let listener = TcpListener::bind(server_address).unwrap();

    for stream in listener.incoming() {
        // let peer_address = tcp_stream.peer_addr()?;

        let mut tcp_stream = stream?;
        let pending_peer_count = &tcp_stream_list.len();

        let mut message: [u8; 1024] = [0; 1024];
        tcp_stream.read(&mut message)?;

        let deserialized_incoming_peer: Peer =
            serde_json::from_str(str::from_utf8(&message)?.trim_end_matches("\0"))?;

        if *pending_peer_count == 0 {
            tcp_stream_list.push(tcp_stream);
            peer_list.push(deserialized_incoming_peer);
        } else {
            //sent back the details of the other peer
            let peer_1 = serde_json::to_string(&peer_list[&peer_list.len() - 1])?;
            let peer_2 = serde_json::to_string(&deserialized_incoming_peer)?;

            tcp_stream.write(peer_1.as_bytes())?;
            tcp_stream_list[*pending_peer_count - 1].write(peer_2.as_bytes())?;

            tcp_stream_list.pop();
            peer_list.push(deserialized_incoming_peer);
        }

        dbg!(&peer_list);
    }

   
  

    Ok(())
}
