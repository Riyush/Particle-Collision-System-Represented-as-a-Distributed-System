module;

#include <iostream>
#include <vector>
#include <map>
#include <string>
#include <cstdlib> // For getenv
#include <sstream>

export module network;

std::map<int, bool> ports_status;

// Helper function to retrieve port numbers from environment variables
void retrieve_ports_from_env(const std::string &prefix, int count) {
    for (int i = 0; i < count; ++i) {
        std::string env_var = prefix + "_" + std::to_string(i);
        const char *port_str = std::getenv(env_var.c_str());
        if (port_str) {
            int port = std::stoi(port_str);
            ports_status[port] = false; // Initialize the port as available
        }
    }
}

// Initialize ports_status once during server startup
export void initialize_ports_status() {
    static bool initialized = false;
    if (!initialized) {
        retrieve_ports_from_env("TCP", 28);
        retrieve_ports_from_env("UDP", 28);
        initialized = true;
    }
}

export int find_next_available_port() {
    for (auto &entry : ports_status) {
        if (!entry.second) {
            entry.second = true;
            return entry.first;
        }
    }
    // If no available port is found
    return -1;
}