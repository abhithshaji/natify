use std::net::{SocketAddr, ToSocketAddrs};
use stunclient::StunClient;
use tokio::net::UdpSocket;
use types::Peer;

pub async fn get_ip_via_stun() -> Peer {
    let local_addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
    let stun_addr = "stun.l.google.com:19302"
        .to_socket_addrs()
        .unwrap()
        .filter(|x| x.is_ipv4())
        .next()
        .unwrap();
    let udp = UdpSocket::bind(&local_addr).await.unwrap();

    let c = StunClient::new(stun_addr);
    let f = c.query_external_address_async(&udp);
    let public_address = f.await.unwrap();

    return Peer::new(public_address.ip(), public_address.port());
}
