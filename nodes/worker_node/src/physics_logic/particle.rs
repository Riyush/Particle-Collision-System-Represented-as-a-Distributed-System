use std::fmt;
use std::ops;
use crate::utils::type_check;

// Physical Constants for the Electron (declared outside the method)
const ELECTRON_MASS: f64 = 9.10938356e-31;  
const ELECTRON_CHARGE: f64 = -1.602176634e-19;
const ELECTRON_SPIN: f64 = 0.5;
const ELECTRON_MAGNETIC_MOMENT: f64 = -9.284764e-24;

// Implement components to represent position, velocity, and accel,
// in 3 component directions
pub struct Components{
    pub x:f64,
    pub y:f64,
    pub z:f64,
}
// basic impl of Component methods like magnitude and normalization
impl Components 
// specify what T could be since it is a generic for now
{
    pub fn magnitude(&self) -> f64 {
        // Calculate the vector magnitude (√(x² + y² + z²)).
        let x_squared: f64 = (self.x * self.x).into();
        let y_squared: f64 = (self.y * self.y).into();
        let z_squared: f64 = (self.z * self.z).into();

        (x_squared + y_squared + z_squared).sqrt()
    }

    pub fn normalize(&self) -> Self {
        // Return a unit vector in the same direction.
        let magnitude = self.magnitude();
        Components {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }
    pub fn scale(&self, scale_factor:f64) -> Self{
        Components {
            x: self.x * scale_factor,
            y: self.y * scale_factor,
            z: self.z * scale_factor,
        }
    }
}
// Don't take ownership of original vectors, but return a new vector
impl ops::Add for &Components{
    type Output = Components;

    fn add(self, other:Self) -> Components{
        Components {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
// Take ownership of both vectors and immediately drop them.
// Use in cases where original vectors are no longer useful
impl ops::Add for Components{
    type Output = Self;

    fn add(self, other:Self) -> Self{
        Components {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
// implementing Display allows me to easily print the struct later on
// I could also use the Debug Trait and {:?} formatter but I defined
// a custom printt format which I like for this use case.
impl fmt::Display for Components{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "({:.1}, {:.1}, {:.1})", self.x, self.y, self.z)
    }
}

//pub struct QuantumNumbers {
//    pub principal: u32, // Principal quantum number (n)
//    pub orbital: u32,   // Orbital angular momentum quantum number (l)
//    pub magnetic: i32,  // Magnetic quantum number (m)
//    pub spin: f64,      // Spin quantum number (+1/2 or -1/2 for electrons)
//}

#[allow(non_camel_case_types)]
pub enum Particle {
    // The variants now has named fields like a struct. 
    Electron{base: BaseParticle}, 
    Muon{base: BaseParticle},
    Neutrino{base: BaseParticle},
    Quark_Up{base: BaseParticle},
    Quark_Down{base: BaseParticle},
    Quark_Left{base: BaseParticle},
    Quark_Right{base: BaseParticle},
    Photon{base: BaseParticle},
    Gluon{base: BaseParticle},
    W_Boson{base: BaseParticle},
    Z_Boson{base: BaseParticle},
    Higgs_Boson{base: BaseParticle},
    Pion{base: BaseParticle},
    Kaon{base: BaseParticle},
}
impl Particle {
    // function to print the enum variant of a particle for debugging only
    pub fn name(&self) {
        match self {
            Particle::Electron{ base: _} => println!("Electron"),
            Particle::Muon{ base: _} => println!("Muon"),
            Particle::Neutrino{ base: _} => println!("Neutrino"),
            Particle::Quark_Up{ base: _} => println!("Up Quark"),
            Particle::Quark_Down{ base: _} => println!("Down Quark"),
            Particle::Quark_Left{ base: _} => println!("Left Quark"),
            Particle::Quark_Right{ base: _} => println!("Right Quark"),
            Particle::Photon{ base: _} => println!("Photon"),
            Particle::Gluon{ base: _} => println!("Gluon"),
            Particle::W_Boson{ base: _} => println!("W Boson"),
            Particle::Z_Boson{ base: _} => println!("Z Boson"),
            Particle::Higgs_Boson{ base: _} => println!("Higgs Boson"),
            Particle::Pion{ base: _} => println!("Pion"),
            Particle::Kaon{ base: _} => println!("Kaon"),
        }
    }
    // funciton to return the base particle struct and particle name
    //from a Particle enum. This function allows for simpler 
    //.base_particle() notation rather than relying on pattern 
    // matching later. I can also use the string in the tuple to identify the
    // particle later on.
    pub fn base_particle(&mut self) -> (&mut BaseParticle, &'static str) {
        match self{
            Particle::Electron{ base} => (base,"Electron"),
            Particle::Muon{ base } => (base,"Muon"),
            Particle::Neutrino{ base} => (base,"Neutrino"),
            Particle::Quark_Up{ base} => (base,"Quark_Up"),
            Particle::Quark_Down{ base} => (base,"Quark_Down"),
            Particle::Quark_Left{ base} => (base,"Quark_Left"),
            Particle::Quark_Right{ base} => (base,"Quark_Right"),
            Particle::Photon{ base} => (base,"Photon"),
            Particle::Gluon{ base} => (base,"Gluon"),
            Particle::W_Boson{ base} => (base,"W_Boson"),
            Particle::Z_Boson{ base} => (base,"Z_Boson"),
            Particle::Higgs_Boson{ base} => (base,"Higgs_Boson"),
            Particle::Pion{ base} => (base,"Pion"),
            Particle::Kaon{ base} => (base,"Kaon"),
        }
    }
}

// Here, I will create traits representing functionalities for specific
// particle variants. This allows me to have generic methods in Particle impl
// and variant specific functionality wrapped in these traits.
//pub trait Electron {
    // example function declaration to best integrate with my .base_particle() function:
    // Note: the input parameter is a tuple which must be destructured
    //pub fn electron_behavior((&self, &str)) -> &str{ 
    //}
//}

// Base Particle struct which contains all properties common to all particles
pub struct BaseParticle {
    pub position: Components,
    pub velocity: Components,
    pub acceleration: Components,
    pub mass: f64,
    pub charge: f64,
    pub spin: f64,
}
impl BaseParticle
{  
    fn new(position: Components, velocity: Components) -> Self {
        BaseParticle {
            position,
            velocity,
            acceleration: Components {x:1.0, y:1.0, z:1.0},
            mass: 0.0,
            charge: 0.0,
            spin: 0.0,
        }
    }
    fn get_position(&self) -> &Components {
        // self.position is still going to be owned by the actual struct
        // because we originally pass a reference and return a reference
        // without conffering new ownership. Also, no mutable references
        // can exist while this returned pointer is in use.
        &self.position
    }
    fn set_position(&mut self, new_pos: Components){
        self.position = new_pos;
    }
    fn get_velocity(&self) -> &Components {
        &self.velocity
    }
    fn set_velocity(&mut self, new_vel: Components){
        self.velocity = new_vel;
    }
    fn get_acceleration(&self) -> &Components {
        &self.acceleration
    }
    fn set_acceleration(&mut self, new_accel: Components){
        self.acceleration = new_accel;
    }
    fn get_mass(&self) -> f64 {
        self.mass
    }
    fn set_mass(&mut self, new_mass: f64){
        self.mass = new_mass;
    }
    fn get_charge(&self) -> f64 {
        self.charge
    }
    fn set_charge(&mut self, new_charge:f64){
        self.charge = new_charge;
    }
    fn get_spin(&self) -> f64 {
        self.spin
    }
    fn set_spin(&mut self, new_spin: f64){
        self.spin = new_spin;
    }
    // function to simulate the particle's movement for a discrete time step
    // of dt. I can simulate any change in time given this dt.
    // Note, I need a time step that is small enough for a smooth animation,
    // but large enough so that there isn't too much load on the network
    // and enough time for the client to receive state updates.
    fn move_simulation(&mut self, dt:f64){
        self.position = &self.position + &(self.velocity.scale(dt));
        self.velocity = &self.velocity + &(self.acceleration.scale(dt));
    }
}
// Implementing the Display Trait allows the .to_string() method which allows
// me to easily print the struct within a format string wiht just a variable name.
// let var1 = BaseParticle{...}; println("{}", var1) - prints because .to_string() is implemented.
impl fmt::Display for BaseParticle{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) ->fmt::Result {
        write!(f, "Base Particle Members: \nposition: {}\nvelocity: {}\nacceleration: {}\nMass: {}\nCharge: {}\nSpin: {}", self.position, self.velocity, self.acceleration, self.mass, self.charge, self.spin)
    }
}

pub fn main() {
    let constructed = BaseParticle::new(Components{x:1.0, y:2.0, z:3.0}, Components{x:-2.0, y:4.0, z:5.0});
    let particle_instance = BaseParticle {
        position: Components {x:1.0, y:2.0, z:3.0},
        velocity: Components {x:-2.0, y:4.0, z:5.0},
        acceleration: Components {x:4.0, y:-3.0, z:-2.0},
        mass: 1.0,
        charge: 1.0,
        spin: 1.0,
    };
    let mut electron_example = Particle::Electron{base:constructed};
    electron_example.name();
    // Note, the type is a tuple with a reference to the base 
    //particle and the particle type as the second argument.
    let (particle, name) = electron_example.base_particle();
    particle.move_simulation(1.0);
    particle.move_simulation(1.0);
    println!("{}", particle);
}