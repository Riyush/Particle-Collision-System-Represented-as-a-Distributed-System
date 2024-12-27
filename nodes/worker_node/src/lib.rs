// Now I have a library crate that is accessible to the binary corresponding
// to main.rs as well as any executables in src/bin

pub mod physics_logic{// Declares the physics_logic module
    // Declare submodules which are files in this case
    pub mod particle;
    pub mod simulation;
    pub mod collision;
} 
mod utils{ // Declares the utils module
    pub mod type_check;
} 