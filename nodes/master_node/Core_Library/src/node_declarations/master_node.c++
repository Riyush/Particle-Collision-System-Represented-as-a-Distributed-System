module;

#include <map>
#include <unordered_map>
#include <string>
#include <vector>
#include <iostream>

export module master_node;
export import worker_node_member;
export import find_new_central_point;
export import network;

export class MasterNode {
public:
    std::unordered_map<Point3D,  WorkerNodeMember, Point3DHash> nodes;
    std::string UDP_Address;
    std::string TCP_Address;
    int space_side_length;

// Constructor
    MasterNode(const std::string& udp, const std::string& tcp)
        : UDP_Address(udp), TCP_Address(tcp), space_side_length(0){}
// Method to get a list of keys from the map
    std::vector<Point3D> getKeys() const {
        std::vector<Point3D> keys;
        keys.reserve(nodes.size()); // Reserve memory to avoid multiple allocations
        for (const auto& pair : nodes) {
            keys.push_back(pair.first); // Add the key (Point3D) to the vector
        }
        return keys;
    }

    void allocate_worker_nodes() {
        // find new central points for nodes in the system
        std::vector<Point3D> new_centers = find_new_central_points(this->space_side_length);

        for (int i = 0; i <new_centers.size(); i++) {
            // get ports for each new worker node
            int tcp_port = find_next_available_port(5000,6000, true);

            int udp_port = find_next_available_port(5000,6000, false);
            // initialize worker node

            //store metadata about the worker node within this master node
            if (this->nodes.find(new_centers[i]) != this->nodes.end()) {
                throw std::runtime_error("Error: A worker node already exists at the given central point!");
            } else {
                this->nodes.emplace(std::move(new_centers[i]), std::move(WorkerNodeMember(udp_port, tcp_port)));
            }
            
        }
        this->space_side_length = this->space_side_length + 2;

}
};