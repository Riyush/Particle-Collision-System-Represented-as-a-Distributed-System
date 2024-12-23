// all modules are declared in lib.rs
// I have to change from crate::physics_logic to worker_node::physics_logic
// because now all the modules are defined in the library crate which crate::
// doesn't have access to. We have to import from the name of the library
// crate which is called worker_node.
use worker_node::physics_logic::particle::main as particle_main; // import it into this file

fn main() {
    particle_main();
    println!("AHHH:");
}