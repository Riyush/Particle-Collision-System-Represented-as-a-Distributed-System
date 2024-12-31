module;

#include <unordered_map>

export module master_node;
export import worker_node_member;

export class MasterNode {
public:
    std::unordered_map<float,  WorkerNodeMember> nodes;
    
};