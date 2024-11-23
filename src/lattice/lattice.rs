use std::collections::HashMap;


enum ProcessorType {
    Bit,
    Spin,
}

// TODO: can the addresses be smaller values?
struct Qubit {
    location: Location,
    bit: bool,
    spin: bool,
} 

struct Location {
    x: f32, 
    y: f32,
}
// age is in ToricCode
// the values for neighboring processors/cells will be grabbed in the methods so there is not redundant data 
struct Processor {
    address: Location,
    processor_type: ProcessorType, 
    syndrome: u32,
    count: Vec<u32>, // history of syndromes
    flipsignal: bool,
}


pub struct LatticeTimeStep {
    qubits: HashMap<Location, Qubit>,
    processors: HashMap<Location, Processor>, 
    // Bit processors are at integer indeces; spin/phase
    // processors are at half-integer indeces
    time: u32, // current time (age)
    size: usize, // same as ToricCode
}

impl LatticeTimeStep {
    fn compute_syndrome(&self, processor: Processor) -> u32 { 
        // given the structure of the
        // lattice, this method is invarient to the type of processor

        // syndrome is the sum of the values of the respective values of the four surrounding
        // qubits 
        let syndrome: u32 = 0;
        let location: Location = processor.address;

        syndrome += self.qubits.get(location.x, location.y + 0.5).syndrome;
        syndrome += self.qubits.get(location.x, location.y - 0.5).syndrome;
        syndrome += self.qubits.get(location.x + 0.5, location.y).syndrome;
        syndrome += self.qubits.get(location.x - 0.5, location.y).syndrome;
        return syndrome;
    }

    fn flip_syndrome(&self, processor: Processor) -> () {
        if processor.processor_type == Bit {
            self.qubits.get(location.x, location.y + 0.5).bit ^= true;
            self.qubits.get(location.x, location.y - 0.5).bit ^= true;
            self.qubits.get(location.x + 0.5, location.y).bit ^= true;
            self.qubits.get(location.x - 0.5, location.y).bit ^= true;
        }
        if processor.processor_type == Spin {
            self.qubits.get(location.x, location.y + 0.5).spin ^= true;
            self.qubits.get(location.x, location.y - 0.5).spin ^= true;
            self.qubits.get(location.x + 0.5, location.y).spin ^= true;
            self.qubits.get(location.x - 0.5, location.y).spin ^= true;
        }
    }
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


