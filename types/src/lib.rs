use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    pub id:String,
    pub public_ip: IpAddr,
    pub port_no: u16,
}

impl Peer {
    pub fn new(id:String,public_ip: IpAddr, port_no: u16) -> Self {
        Self { id,public_ip, port_no }
    }
}