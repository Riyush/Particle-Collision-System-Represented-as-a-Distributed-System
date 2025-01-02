module;

#include <iostream>
#include <stdexcept>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>

export module network;

export int find_next_available_port(int start_port, int end_port, bool is_tcp) {
    for (int port = start_port; port <= end_port; ++port) {
        int sock = socket(AF_INET, is_tcp ? SOCK_STREAM : SOCK_DGRAM, 0);
        if (sock < 0) {
            throw std::runtime_error("Failed to create socket");
        }

        sockaddr_in addr{};
        addr.sin_family = AF_INET;
        addr.sin_port = htons(port);
        addr.sin_addr.s_addr = INADDR_ANY;

        // Try to bind the socket
        if (bind(sock, reinterpret_cast<sockaddr*>(&addr), sizeof(addr)) == 0) {
            close(sock); // Close the socket after successful bind
            return port; // Return the available port
        }

        close(sock); // Close the socket if binding fails
    }
    
    throw std::runtime_error("No available ports in the specified range");
}