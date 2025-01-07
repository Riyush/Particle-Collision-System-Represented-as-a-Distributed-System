module;
#include <string>
#include <iostream>

export import point_3D;
export module docker_command;

std::string image_name("particle_node");

export std::string generate_docker_build() {
    std::string dockerfile_path = "/app/src/nodes/master_node/Core_Library/src/utils/Dockerfile";  // Full path to the Dockerfile
    std::string context_dir = "/app/src/nodes";  // Context directory

    return "docker build -f " + dockerfile_path + " " + context_dir + " -t " + image_name;
}
// pass in important variables as env Vars in worker node's container
export std::string generate_docker_run(std::string& tcp_master, std::string& udp_master, int tcp_worker, int udp_worker, const Point3D& central_point) {
    //convert master_node addresses into just their ports which are last 4 chars
    tcp_master = tcp_master.substr(10,4);
    udp_master = udp_master.substr(10,4);

    //convert int ports into string ports for env variables
    std::string tcp_address_worker =std::to_string(tcp_worker);
    std::string udp_address_worker =std::to_string(udp_worker);

    // Get x,y,z points
    float x = central_point.x;
    float y = central_point.y;
    float z = central_point.z;

    // Generate string of all environmental variables
    std::string env_vars = "-e TCP_MASTER=" + tcp_master + " -e UDP_MASTER=" + udp_master +
                           " -e TCP_WORKER=" + tcp_address_worker + " -e UDP_WORKER=" + udp_address_worker +
                           " -e X_COORD=" + std::to_string(x) + " -e Y_COORD=" + std::to_string(y) + " -e Z_COORD=" + std::to_string(z);

    // Generate string to pass ports
    std::string ports;
    if (tcp_worker == udp_worker) {
        ports = "-p " + std::to_string(tcp_worker) + ":" + std::to_string(tcp_worker);
    } else {
        ports = "-p " + std::to_string(tcp_worker) + ":" + std::to_string(tcp_worker) +
                " -p " + std::to_string(udp_worker) + ":" + std::to_string(udp_worker);
    }

    return "docker run " + ports + " " + env_vars + " " + image_name;
}