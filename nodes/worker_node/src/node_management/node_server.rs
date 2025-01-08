use crate::utils::type_check;
use std::fmt;
use std::net::{IpAddr, SocketAddr, Ipv4Addr, TcpListener};

pub struct NodeServer {
    pub worker_TCP:SocketAddr,
    pub worker_UDP:SocketAddr,
    pub master_TCP:SocketAddr,
    pub master_UDP:SocketAddr,
}
impl NodeServer{
    pub fn new(worker_TCP:SocketAddr, worker_UDP:SocketAddr, master_TCP:SocketAddr, master_UDP:SocketAddr) -> Self{
        Self{
            worker_TCP,
            worker_UDP,
            master_TCP,
            master_UDP,
        }
    }
}