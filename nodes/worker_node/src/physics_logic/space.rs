// import all public modules in particle
use crate::physics_logic::particle::*;
use std::fmt;
use std::collections::HashMap;

pub struct Space {
    pub particles: Vec<Particle>,
    pub faces: Vec<Face>,
    pub central_point_in_cube: Components,
}
// Every space struct will represent a 2x2x2 3d space
// The first struct will be a 2x2x2 space centered at (1,1,1) -> Makes math easier

impl Space {
    pub fn new(central_point_in_cube: Components) -> Self{
        let mut particles: Vec<Particle> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();
        //start a loop that creates all 6 sides
        for num in (0..6).rev(){ // no point of .rev, it's just exercise using the method
            match num {
                5 => { // Top Face Z dimension
                    let central_point_on_face_plane = &central_point_in_cube + &Components{x:0.0, y:0.0, z:1.0};
                    let mut top_face = Face::new(true, Dimension::Z, central_point_on_face_plane.z, central_point_on_face_plane);
                    faces.push(top_face);
                }
                4 => { // Bottom Face Z dimension
                    let central_point_on_face_plane = &central_point_in_cube + &Components{x:0.0, y:0.0, z:-1.0};
                    let mut bottom_face = Face::new(true, Dimension::Z, central_point_on_face_plane.z, central_point_on_face_plane);
                    faces.push(bottom_face);
                }
                3 => { // Right Face X Dimension
                    let central_point_on_face_plane = &central_point_in_cube +  &Components{x:1.0, y:0.0, z:0.0};
                    let mut right_x_face = Face::new(true, Dimension::X, central_point_on_face_plane.x, central_point_on_face_plane);
                    faces.push(right_x_face);
                }
                2 => { // Left Face X Dimension
                    let central_point_on_face_plane = &central_point_in_cube + &Components{x:-1.0, y:0.0, z:0.0};
                    let mut left_x_face = Face::new(true, Dimension::X, central_point_on_face_plane.x, central_point_on_face_plane);
                    faces.push(left_x_face);
                }
                1 => { // "forward" Face Y Dimension
                    let central_point_on_face_plane = &central_point_in_cube + &Components{x:0.0, y:1.0, z:0.0};
                    let mut forward_y_face = Face::new(true, Dimension::Y, central_point_on_face_plane.y, central_point_on_face_plane);
                    faces.push(forward_y_face);
                }
                0 => { // "back" Face Y Dimsions 
                    let central_point_on_face_plane = &central_point_in_cube + &Components{x:0.0, y:-1.0, z:0.0};
                    let mut back_y_face = Face::new(true, Dimension::Y, central_point_on_face_plane.y, central_point_on_face_plane);
                    faces.push(back_y_face);
                }
                _ => unreachable!(), // Catch-all for impossible cases
            };
        }
        Space {
            particles,
            faces,
            central_point_in_cube,
        }
    }
}
// An implementation of Space.to_string() which will print a space variable and it's important properties.
impl fmt::Display for Space{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut face_strings = vec![];
        for face in &self.faces{
            face_strings.push(face.to_string());
        }
        write!(f, "Space Center: {}\n FACES\nTop Face: {}\nBottom Face: {}\nRight_X_Face: {}\nLeft_X_Face: {}\nForward_Y_Face: {}\n Back_Y_Face: {}", self.central_point_in_cube, face_strings[0], face_strings[1], face_strings[2], face_strings[3], face_strings[4], face_strings[5])
    }
}
pub struct Face {
    bounded: bool, // Is the face open or bounded
    constant_dimension: Dimension,  // Can be 'x', 'y', 'z'
    constant_plane_coordinate: f64, // The value of the dimension of the plane that is contant throughout the plane
    central_point_on_face_plane: Components,  // since all faces have constant length
}
impl Face {
    fn new(bounded: bool, constant_dimension:Dimension, constant_plane_coordinate:f64, central_point_on_face_plane:Components) -> Self{
        Face {
            bounded,
            constant_dimension,
            constant_plane_coordinate,
            central_point_on_face_plane,
        }
    }
}
impl fmt::Display for Face{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Face Center: {}\n Is bounded?: {}\n Constant Dimension: {}\n Constant Dimension Value: {}", self.central_point_on_face_plane, self.bounded, self.constant_dimension, self.constant_plane_coordinate)
    }
}
pub enum Dimension {
    X,
    Y,
    Z,
}
impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dimension::X => write!(f, "X"),
            Dimension::Y => write!(f, "Y"),
            Dimension::Z => write!(f, "Z"),
        }
    }
}
// main function for debugging purposes called in binary script simulation_debug
pub fn space_main() {
    let center_first_cube = Components{x:1.0,y:1.0,z:1.0};
    let mut first_cube = Space::new(center_first_cube);
    let center_second_cube = Components{x:3.0, y:1.0,z:1.0};
    let mut second_cube = Space::new(center_second_cube);
    println!("{}", first_cube);
    println!("{}", second_cube);
}