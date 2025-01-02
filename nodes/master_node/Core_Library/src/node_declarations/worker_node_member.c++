module;

#include <iostream>
#include <string>

export module worker_node_member;

export class WorkerNodeMember {
public:
    std::string UDP_Address;
    std::string TCP_Address;

    // Constructor to accept port numbers and construct full addresses
    WorkerNodeMember(int udp_port, int tcp_port)
        : UDP_Address("127.0.0.1:" + std::to_string(udp_port)),
          TCP_Address("127.0.0.1:" + std::to_string(tcp_port)) {}
    
    // Overload the << operator to print the worker node members
    friend std::ostream& operator<<(std::ostream& os, const WorkerNodeMember& node){
        os << "Node Details\nTCP Address: "<< node.TCP_Address << "\nUDP Address: " << node.UDP_Address;
        return os;
    }
};