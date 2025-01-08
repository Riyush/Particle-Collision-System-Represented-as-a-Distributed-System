use std::env;

//use standard library to pull all environmental variables
pub fn get_important_env_vars()-> (String, String, String, String){
    let mut tcp_master:String = String::new();
    let mut udp_master:String = String::new();
    let mut tcp_worker:String = String::new();
    let mut udp_worker:String = String::new();

    
    match env::var("TCP_MASTER") {
        Ok(val) => tcp_master = val.clone(),
        Err(e) => println!("{}", e),
    }
    match env::var("UDP_MASTER") {
        Ok(val) => udp_master= val.clone(),
        Err(e) => println!("{}", e),
    }
    match env::var("TCP_WORKER") {
        Ok(val) => tcp_worker = val.clone(),
        Err(e) => println!("{}", e),
    }
    match env::var("UDP_WORKER") {
        Ok(val) => udp_worker = val.clone(),
        Err(e) => println!("{}", e),
    }
    // Return all important env variables as a tuple to be traversed
    (tcp_worker, udp_worker, tcp_master, udp_master)
}