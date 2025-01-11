use std::env;
//use standard library to pull all environmental variables
pub fn get_important_env_vars()-> (String, String, String, String, f64, f64, f64){
    // get env vars for server setup
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

    // get env vars for space setup
    let mut x_center_coord:f64 = 0.0;
    let mut y_center_coord:f64 = 0.0;
    let mut z_center_coord:f64 = 0.0;

    match env::var("X_COORD") {
        Ok(val) => x_center_coord = val.parse::<f64>().unwrap_or_else(|_| {
            eprintln!("Error parsing X_COORD as f64");
            0.0
        }),
        Err(e) => println!("{}", e),
    }
    match env::var("Y_COORD") {
        Ok(val) => y_center_coord = val.parse::<f64>().unwrap_or_else(|_| {
            eprintln!("Error parsing Y_COORD as f64");
            0.0
        }),
        Err(e) => println!("{}", e),
    }
    match env::var("Z_COORD") {
        Ok(val) => z_center_coord = val.parse::<f64>().unwrap_or_else(|_| {
            eprintln!("Error parsing Z_COORD as f64");
            0.0
        }),
        Err(e) => println!("{}", e),
    }

    
    // Return all important env variables as a tuple to be traversed
    (tcp_worker, udp_worker, tcp_master, udp_master, x_center_coord, y_center_coord, z_center_coord)
}