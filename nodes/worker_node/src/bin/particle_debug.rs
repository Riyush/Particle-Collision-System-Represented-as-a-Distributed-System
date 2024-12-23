// Since we made a library crate, I can more easily access that crate
// from just the scope of this singular file which is its own crate

// Import from the library crate using its root name: worker_node
use worker_node::physics_logic::particle;

pub fn main() {
    particle::main();
}