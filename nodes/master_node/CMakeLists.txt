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
            Core_library/src/archive.c++
)
# Add include directories to help the compiler locate module files
target_include_directories(Core_Library PUBLIC Core_library/src)

# Application setup
add_executable(app bin/app.c++)
target_link_libraries(app PRIVATE Core_Library)
