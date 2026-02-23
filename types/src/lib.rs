use std::net::IpAddr;

pub struct Peer {
    pub public_ip: IpAddr,
    pub port_no: u16,
}

impl Peer {
    pub fn new(public_ip: IpAddr, port_no: u16) -> Self {
        Self { public_ip, port_no }
    }
}