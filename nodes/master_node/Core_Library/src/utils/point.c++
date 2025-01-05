module;

#include <iostream>
#include <string>

export module point_3D;

export class Point3D { 

public:
    float x, y, z;
    // Constructor
    Point3D(float x, float y, float z) : x(x), y(y), z(z) {}

    // Accessors
    float getX() const { return x; }
    float getY() const { return y; }
    float getZ() const { return z; }

    // Mutators
    void setX(float value) { x = value; }
    void setY(float value) { y = value; }
    void setZ(float value) { z = value; }

    // Overload + for point addition, takes in another point 
    // and returns a new point representing the result of adding the 2 points
    // const -> keyword for not mutable, Point3D& is a reference 
    // Also, note type is declared left of variable name unlike rust
    // const at the end ensures we don't modify the Point3d this add operation is called on.
    Point3D operator+(const Point3D& other) const {
        return Point3D(x + other.x, y + other.y, z + other.z);
    }

    // Overload the << operator to print the class:
    // std::ostream& os: Represents the output stream to which the data is written.

    friend std::ostream& operator<<(std::ostream& os, const Point3D& p){
        os << "Point:\nX: "<< p.x << "\nY: " << p.y << "\nZ: " << p.z << "\n";
        return os;
    }
    // Equality operator
    bool operator==(const Point3D& other) const {
        return (x == other.x) && (y == other.y) && (z == other.z);
    }
};

// Custom hash function for Point3D
export struct Point3DHash {
    size_t operator()(const Point3D& p) const {
        // Combine the hash of x, y, and z
        size_t hx = std::hash<float>{}(p.getX());
        size_t hy = std::hash<float>{}(p.getY());
        size_t hz = std::hash<float>{}(p.getZ());

        // Combine the individual hash values using XOR and shifting
        return hx ^ (hy << 1) ^ (hz << 2);
    }
};
bool operator==(const Point3D& p1, const Point3D& p2) {
    return p1.getX() == p2.getX() && p1.getY() == p2.getY() && p1.getZ() == p2.getZ();
}