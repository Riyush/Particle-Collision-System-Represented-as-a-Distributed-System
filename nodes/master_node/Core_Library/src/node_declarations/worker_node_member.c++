module;

#include <string>

export module worker_node_member;

export class WorkerNodeMember {
public:
    std::string UDP_Address;
    std::string TCP_Address;

    // Constructor?
     WorkerNodeMember(const std::string& udp = "0.0.0.0", const std::string& tcp = "0.0.0.0")
        : UDP_Address(udp), TCP_Address(tcp) {}
}