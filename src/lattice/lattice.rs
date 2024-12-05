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

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

// TODO: can the addresses be smaller values?
struct Qubit {
    location: Location,
    bit: bool,
    spin: bool,
} 

struct Location {
    x: u32, 
    y: u32,
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
    // qubits are at (odd, even) and (even, odd) indeces 
    processors: HashMap<Location, Processor>,
    // Bit processors are at (even, even) indeces; 
    // spin/phase processors are at (odd,odd) indeces
    time: u32, // current time (age)
    size: u32, 
}

impl LatticeTimeStep {
    fn new() {
        for y in 0..=size {
            for x in 0..=size {
                match (x % 2, y % 2) {
                    // bit processor
                    (0, 0) => Processor {
                        address: Location {x, y},
                        processor_type: ProcessorType::Bit,
                        syndrome: 0,
                        count: Vec::new(),
                        flipsignal: false,
                    },

                    // spin processor 
                    (1, 1) => Processor {
                        address: Location {x, y},
                        processor_type: ProcessorType::Spin,
                        syndrome: 0,
                        count: Vec::new(),
                        flipsignal: false,
                    },

                    // Qubit 
                    _ => Qubit {
                        location: Location {x, y},
                        bit: false,
                        spin: false,
                    },
                }
            }
        }
    }
    // gets the qubit adjacent to a PROCESSOR in the direction specified
    fn get_adjacent_qubit(&self, direction: Cardinal, processor: Processor) -> Qubit {
        let new_location: Location = match &direction {
            Cardinal::North => Location { 
                x: processor.x, 
                y: (processor.y + 1) % self.size, 
            },
            Cardinal::East => Location { 
                x: (processor.x + 1) % self.size, 
                y: processor.y,
            },
            Cardinal::South => Location { 
                x: processor.x, 
                y: (processor.y - 1) % self.size,
            },
            Cardinal::West => Location { 
                x: (processor.x - 1) % self.size, 
                y: processor.y,
            },
        };
        return self.qubits.get(new_location);
    }

    fn compute_syndrome<ProcessorType>(&self, processor: Processor) -> u32 { 
        // syndrome is the sum of the values of the respective values of the four surrounding
        // qubits 
        let syndrome: u32 = [Cardinal::North, Cardinal::East, Cardinal::South, Cardinal::West]
            .iter()
            .map(|direction| match ProcessorType {
                ProcessorType::Bit => self.get_adjacent_qubit(direction, processor).bit as u32,
                ProcessorType::Spin => self.get_adjacent_qubit(direction, processor).spin as u32
            })
            .sum();
        return syndrome;
    }

    fn flip_syndrome(&self, processor: Processor) -> () {
        let cardinals: Vec<Cardinal> = [Cardinal::North, Cardinal::East, Cardinal::South, Cardinal::West];
        match processor.processor_type {
            Bit => &cardinals 
                .iter()
                .map(|direction| self.get_adjacent_qubit(direction, processor).bit ^= true),
            Spin => &cardinals
                .iter()
                .map(|direction| self.get_adjacent_qubit(direction, processor).spin ^= true),
        };
    }
    
    // gets the adjacent processor to another processor in a specified cardinal or ordinal
    // direction
    fn get_adjacent_processor(&self, direction: Direction, processor: Processor) -> Processor {
        let new_location: Location = match &direction {
            Direction::North => Location { 
                x: processor.x, 
                y: (processor.y + 2) % self.size  
            },
            Direction::NorthEast => Location {
                x: (processor.x + 2) % self.size,
                y: (processor.y + 2) % self.size
            },
            Direction::East => Location {
                x: (processor.x + 2) % self.size,
                y: processor.y
            },
            Direction::SouthEast => Location {
                x: (processor.x + 2) % self.size,
                y: (processor.y - 2) % self.size
            },
            Direction::South => Location {
                x: processor.x,
                y: (processor.y - 2) % self.size
            },
            Direction::SouthWest => Location {
                x: (processor.x - 2) % self.size,
                y: (processor.y - 2) % self.size
            },
            Direction::West => Location {
                x: (processor.x - 2) % self.size,
                y: processor.y
            },
            Direction::NorthWest => Location {
                x: (processor.x - 2) % self.size,
                y: (processor.y + 2) % self.size
            },
        };    
        return self.processors.get(new_location);
    } 
}

pub struct Lattice { // NOTE: the lattice is now incremented by integers to not have floating point
    // rounding errors
    size: usize, // total size
    total_time_steps: u32, // TODO: may be unnecessary
    steps: Vec<LatticeTimeStep>, // Vector of ToricCodeTimeStep's, each representing the state of the toric code at a point in time
}


impl Lattice {
    pub fn new(qubits: usize) -> Self {
        Lattice {
            // size is the side length, which is twice the number of qubits minus 1
            size: 2*qubits - 1, //qubits must be greater than zero 
            total_time_steps: 0,
            steps: Vec::new(),
            start(),
            
        }
    }

    pub fn start() -> () {
         //make the initial lattice 
        // create a new LatticeTimeStep 
       

        //add to steps
   
    }

    pub fn increment_time() -> () {
        //add new LatticTimeStep to the steps
        total_time_steps += 1;


    }
    
    //TODO: make methods that add a new thing without having to create a new lattice
    //different types of increment_time?

}

// TODO: add test cases
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_compute_syndrome() {
    //
    // }
}
