use std::collections::HashMap;


enum ProcessorType {
    Bit,
    Spin,
}

enum Cardinal {
    North,
    East,
    South,
    West,
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
    size: f32, 
}

impl LatticeTimeStep {
    // gets the qubit adjacent to a PROCESSOR in the direction specified
    fn getAdjacentQubit(&self, direction: Cardinal, processor: Processor) -> Qubit {
        let new_location: Location = match &direction {
            Cardinal::North => Location { 
                x: processor.x, 
                y: processor.y + 0.5 % self.size, 
            },
            Cardinal::East => Location { 
                x: processor.x + 0.5 % self.size, 
                y: processor.y,
            },
            Cardinal::South => Location { 
                x: processor.x, 
                y: processor.y - 0.5 % self.size,
            },
            Cardinal::West => Location { 
                x: processor.x - 0.5 % self.size, 
                y: processor.y,
            },
        };
        return self.qubits.get(new_location);
    }

    fn computeSyndrome(&self, processor: Processor) -> u32 { 
        // syndrome is the sum of the values of the respective values of the four surrounding
        // qubits 
        let syndrome: u32 = [Cardinal::North, Cardinal::East, Cardinal::South, Cardinal::West]
            .iter()
            .map(|direction| self.getAdjacentQubit(direction, processor))
            .sum();
        return syndrome;
    }

    fn flipSyndrome(&self, processor: Processor) -> () {
        let cardinals: Vec<Cardinal> = [Cardinal::North, Cardinal::East, Cardinal::South, Cardinal::West];
        match processor.processor_type {
            Bit => &cardinals 
                .iter()
                .map(|direction| self.getAdjacentQubit(direction, processor).bit ^= true),
            Spin => &cardinals
                .iter()
                .map(|direction| self.getAdjacentQubit(direction, processor).spin ^= true),
        };
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


