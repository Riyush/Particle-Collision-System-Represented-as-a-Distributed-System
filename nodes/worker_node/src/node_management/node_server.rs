use crate::utils::type_check;
use std::fmt;
use std::io::{prelude::*, BufReader};
use std::net::{IpAddr, SocketAddr, Ipv4Addr, TcpListener, TcpStream};

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
    pub fn start_server(&self){
        // This will bind a Socket to listen for TCP communications using worker_TCP
        // Note this starts a live server, so it needss to be implemented on its own thread
        let listener = TcpListener::bind(self.worker_TCP).unwrap();
        println!("Server listening on {}", self.worker_TCP);
        // The stream represents a lisst of incoming requests to the server
        for stream in listener.incoming() {
            let mut stream = stream.unwrap(); 
            // Note unwrap simply returns the error and terminates the server
            // If we want to handle errors a specific way within our server, we need to create that logic
            self.handle_connection(stream);
            
    } 
    }
    pub fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);
        // for now we form a vec to represent the information sent to our server
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

            let response = "HTTP/1.1 200 OK\r\n\r\n";

            stream.write_all(response.as_bytes()).unwrap();
    }
    
}