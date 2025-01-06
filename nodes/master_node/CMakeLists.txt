cmake_minimum_required(VERSION 3.28)
project(ParticleCollisionSystem)

# Set the version of C++ for the project
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Enable support for C++ modules
set(CMAKE_EXPERIMENTAL_CXX_MODULE_CMAKE_API "ON")
set(CMAKE_EXPERIMENTAL_CXX_MODULE_DYNDEP "ON")
set(CMAKE_CXX_SCAN_FOR_MODULES ON)

# Core Library setup
add_library(Core_Library STATIC)
target_sources(Core_Library
    PUBLIC
        FILE_SET CXX_MODULES FILES
            Core_library/src/node_declarations/master_node.c++
            Core_library/src/node_declarations/worker_node_member.c++
            Core_library/src/utils/point.c++
            Core_library/src/worker_node_setup/find_central_point_of_new_worker_node.c++
            Core_library/src/worker_node_setup/get_network_ports.c++
            Core_library/src/worker_node_setup/create_docker_command.c++
            Core_library/src/worker_node_setup/execute_system_command.c++


)
# Add include directories to help the compiler locate module files
target_include_directories(Core_Library PUBLIC Core_library/src)
target_include_directories(Core_Library PUBLIC Core_library/src/node_declarations)
target_include_directories(Core_Library PUBLIC Core_library/src/utils)
target_include_directories(Core_Library PUBLIC Core_library/src/worker_node_setup)

# Application setup - needs to encapsulate all logic of starting the application
add_executable(app bin/app.c++)
target_link_libraries(app PRIVATE Core_Library)

# New executable specifically for debugging
#add_executable(debug bin/debug.c++)
#target_link_libraries(debug PRIVATE Core_Library)
