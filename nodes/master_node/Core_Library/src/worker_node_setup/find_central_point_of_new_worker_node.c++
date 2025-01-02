module;

#include <string>
#include <vector>
#include <map>
#include <stdexcept>
#include <iostream>

using namespace std;

export module find_new_central_point;
export import point_3D;

export std::map<int, Point3D> get_all_central_points(){
    std::map<int, Point3D> allocated_nodes;
    allocated_nodes.emplace(0, Point3D(1, 1, 1));
    //start of 4x4x4 space
    allocated_nodes.emplace(1, Point3D(3, 1, 1));
    allocated_nodes.emplace(2, Point3D(1, 3, 1));
    allocated_nodes.emplace(3, Point3D(1, 1, 3));
    allocated_nodes.emplace(4, Point3D(1, 3, 3));
    allocated_nodes.emplace(5, Point3D(3, 1, 3));
    allocated_nodes.emplace(6, Point3D(3, 3, 1));
    allocated_nodes.emplace(7, Point3D(3, 3, 3));
    //start of 6x6x6 space
    allocated_nodes.emplace(8, Point3D(5, 1, 1));
    allocated_nodes.emplace(9, Point3D(5, 3, 1));
    allocated_nodes.emplace(10, Point3D(5, 5, 1));
    allocated_nodes.emplace(11, Point3D(3, 5, 1));
    allocated_nodes.emplace(12, Point3D(1, 5, 1));
    allocated_nodes.emplace(13, Point3D(5, 1, 3));
    allocated_nodes.emplace(14, Point3D(5, 3, 3));
    allocated_nodes.emplace(15, Point3D(5, 5, 3));
    allocated_nodes.emplace(16, Point3D(3, 5, 3));
    allocated_nodes.emplace(17, Point3D(1, 5, 3));
    allocated_nodes.emplace(18, Point3D(1, 1, 5));
    allocated_nodes.emplace(19, Point3D(3, 1, 5));
    allocated_nodes.emplace(20, Point3D(1, 3, 5));
    allocated_nodes.emplace(21, Point3D(3, 3, 5));
    allocated_nodes.emplace(22, Point3D(1, 5, 5));
    allocated_nodes.emplace(23, Point3D(5, 1, 5));
    allocated_nodes.emplace(24, Point3D(5, 3, 5));
    allocated_nodes.emplace(25, Point3D(3, 5, 5));
    allocated_nodes.emplace(26, Point3D(5, 5, 5));
    return allocated_nodes;
}

export vector<Point3D> find_new_central_points(int space_side_length) {
    std::map<int, Point3D> allocated_nodes = get_all_central_points();
    std::vector<Point3D> new_points;

    if (allocated_nodes.count(0) == 0) {
    std::cerr << "Key 0 does not exist in allocated_nodes!" << std::endl;
    }

    if (space_side_length == 0) {
        new_points.push_back(std::move(allocated_nodes.at(0)));
        allocated_nodes.erase(0); // Remove the element from the map to complete the transfer of ownership.
    }
    else if (space_side_length == 2){
        for (int i = 1; i < 8; i++){
            new_points.push_back(std::move(allocated_nodes.at(i)));
            allocated_nodes.erase(i); 
        }
    }
    else if (space_side_length == 4){
        for (int i = 8; i < 27; i++){
            new_points.push_back(std::move(allocated_nodes.at(i)));
            allocated_nodes.erase(i); 
        }
    }
    else {
        throw std::invalid_argument("Unsupported space_side_length");
    }
    return new_points;
};
