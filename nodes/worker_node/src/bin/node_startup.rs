use worker_node::physics_logic::space;
use worker_node::node_management::node_server::NodeServer;
use worker_node::utils::env_vars::{get_important_env_vars};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
fn main(){
    //get env vars
    let vars:(String, String, String, String) = get_important_env_vars();
    println!("{} | {} | {} | {}", vars.0, vars.1, vars.2, vars.3);
    //use env vars instantiate a NodeServer struct which contains
    // addresses and methods to conduct communcations.
    let tcp_worker = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.0.parse::<u16>().unwrap());
    let udp_worker = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.1.parse::<u16>().unwrap()); 
    let tcp_master = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.2.parse::<u16>().unwrap());
    let udp_master = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.3.parse::<u16>().unwrap());

    // instantiate a NodeServer to store all necessary addresses
    let server = NodeServer::new(tcp_worker, udp_worker, tcp_master, udp_master);
    println!("{}", server.master_TCP);
}