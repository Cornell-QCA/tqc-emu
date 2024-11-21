use lattices::{map_union::MapUnion, VecUnion};


enum QubitState {
    Temporary,
    // TODO: determine varients
    //      1
    //      0
    //      error?
}

// TODO: can the addresses be smaller values?
struct Qubit {
    location: Location,
    state: QubitState,
} 

struct Location {
    x: f32, 
    y: f32,
}

enum Address {
    Vertex(Location), // one of these is at integer indeces and the other is at half integer
    Plaquette(Location),
}


// age is in ToricCode
// the values for neighboring processors/cells will be grabbed in the methods so there is not redundant data 
struct Processor<Varient> {
    address: Address::Varient,
    syndrome: u32,
    count: Vec<u32>, // history of syndromes
    flipsignal: bool,
}

pub struct LatticeTimeStep {
    spin_qubits: MapUnion<Location, QubitState>,
    bit_flip_processors: MapUnion<Location, Processor<Vertex>>, // TODO: THESE MAY NEED TO BE SWITCHED (vertex/plaquette)
    phase_processors: MapUnion<Location, Processor<Plaquette>>,
    time: u32, // current time (age)
    size: usize, // same as ToricCode
}

pub struct Lattice {
    size: usize, // total size
    total_time_steps: u32, // TODO: may be unnecessary
    steps: VecUnion<LatticeTimeStep>, // Vector of ToricCodeTimeStep's, each representing the state of the toric code at a point in time
}


impl Lattice {
    pub fn new(qubits: usize) -> Self {
        Lattice {
            size: qubits * qubits, //qubits must be greater than zero
            total_time_steps: 0,
            steps: VecUnion::new(),
            start(),
            
        }
    }

    pub fn start() -> () {
        //make the initial lattice (new lattice_time_step)
        
        for y in 0..size {
            for x in 0..size {
                //add bit_flip (x,y)
                //add qubit (x+0.5, y)
                //add qubit (x, y+0.5)
                //spin_flip (x+0.5, y+0.5)
            }
        }

        //add to steps
   
    }

    pub fn increment_time() -> () {
        //add new LatticTimeStep to the steps
        total_time_steps += 1;


    }
    
    //TODO: make methods that add a new thing without having to create a new lattice
    //different types of increment_time?

}


