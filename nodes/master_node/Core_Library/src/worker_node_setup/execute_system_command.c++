module;

#include <string>
#include <iostream>
#include <cstdlib>


export module system_command;

export void execute_command(const std::string& command){
    // Convert std::string to const char* using c_str()
    int result = system(command.c_str());
    // Error checking
    if (result == 0) {
        std::cout << "Command executed successfully." << std::endl;
    } else {
        std::cerr << "Error executing command." << std::endl;
    }
}