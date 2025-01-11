use worker_node::physics_logic::space::*;
use worker_node::physics_logic::particle::*;
use worker_node::node_management::node_server::NodeServer;
use worker_node::utils::env_vars::{get_important_env_vars};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
fn main(){
    //get env vars
    let vars:(String, String, String, String, f64, f64, f64) = get_important_env_vars();
    //use env vars instantiate a NodeServer struct which contains
    // addresses and methods to conduct communcations.
    let tcp_worker = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.0.parse::<u16>().unwrap());
    let udp_worker = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.1.parse::<u16>().unwrap()); 
    let tcp_master = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.2.parse::<u16>().unwrap());
    let udp_master = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), vars.3.parse::<u16>().unwrap());

    // Initialize the space struct by creating using new constructor that only requires a central point
    let x_center_coord: f64 = vars.4;
    let y_center_coord: f64 = vars.5;
    let z_center_coord: f64 = vars.6;
    let central_point:Components = Components{x:x_center_coord, y:y_center_coord, z: z_center_coord};
    let mut worker_node_space_partition = Space::new(central_point);

    // instantiate a NodeServer to store all necessary addresses
    let server = NodeServer::new(tcp_worker, udp_worker, tcp_master, udp_master);
    println!("{}", server.worker_TCP);

    // Initiailize the TCP and UDP Listeners using NodeServer
    //server.start_server();
}