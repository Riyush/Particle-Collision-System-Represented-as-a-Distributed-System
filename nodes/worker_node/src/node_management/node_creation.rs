use std::net::UdpSocket;

pub fn create_worker_node(){
    /* The master node will have a function that creates the container
    for a worker node. That container will have a copy of the worker_node
    directory. This function will be called within the context of the newly
    instantiated container to initiate the space struct, set up a UDP socket,
    and notify the master node that a worker node has been successfully
    created.
     */
}
fn main() {
    // sample code of everything this file needs to do.
    let worker_id = "worker_1"; // Example: This could be passed as an argument
    let master_address = "127.0.0.1:8080"; // Master node's address

    // Initialize worker
    let space = initialize_space();
    println!("Worker node {} initialized with space: {:?}", worker_id, space);

    // Set up UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");
    println!("Worker node listening on: {:?}", socket.local_addr().unwrap());

    // Notify master node
    let msg = format!("READY:{}", worker_id);
    socket
        .send_to(msg.as_bytes(), master_address)
        .expect("Failed to send READY message to master");
}

fn initialize_space() -> (i32, i32, i32) {
    // Replace with your actual Space struct logic
    (0, 0, 0)
}