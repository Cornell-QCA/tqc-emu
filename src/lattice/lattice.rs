use lattices::{map_union::MapUnion, VecUnion};



enum Orientation {
    Vertical,
    Horizontal,
}

enum QubitState {
    // TODO: determine varients
    //      1
    //      0
    //      error?
}

// TODO: can the addresses be smaller values?
struct Edge {
    x: u32,
    y: u32,
    orientation: Orientation,
    // I think indexing from some origin, like the xy plane, and having each (x,y) pair correspond to two edges: 
    // the one above the vertex and to the right of the vertex (current struct setup)

} 


enum Address {
    Vertex {
        x: u32,
        y: u32,
    },
    Plaquette {
        x: u32,
        y: u32,
    },
}


// age is in ToricCode
// the values for neighboring processors/cells will be grabbed in the methods so there is not redundant data 
struct Processor<Varient> {
    address: Address::Varient,
    syndrome: u32,
    count: Vec<u32>, // history of syndromes
    flipsignal: bool,
}

pub struct ToricCodeTimeStep {
    spin_qubits: MapUnion<Edge, QubitState>,
    bit_flip_processors: MapUnion<Vertex, Processor<Vertex>>, // TODO: THESE MAY NEED TO BE SWITCHED (vertex/plaquette)
    phase_processors: MapUnion<Plaquette, Processor<Plaquette>>,
    size: usize, // same as ToricCode
}

pub struct ToricCode {
    size: usize, // total size
    total_time_steps: u32, // TODO: may be unnecessary
    time: u32, // current time (age)
    step: VecUnion<ToricCodeTimeStep>, // Vector of ToricCodeTimeStep's, each representing the state of the toric code at a point in time
}