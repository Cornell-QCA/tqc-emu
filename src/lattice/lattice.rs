use std::collections::HashMap;
use math::round::floor;

const Q: u32 = 16; // colony size 
const U: u32 = 400; // work period
const f_c: f32 = 4/5; // threshold for a cell's count
const f_n: f32 = 1/5; // threshold for a neighboring cell's count

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
    // Center, // regards the implement_flip_syndrome optimization
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

// regards the implement_flip_syndrome optimization
// enum Action {
//     DoNothing,
//     FlipNorth,
//     FlipEast,
//     FlipSouth,
//     FlipWest,
// }

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
    syndrome: bool,
    count: Vec<u32>, // history of syndromes
    flipsignal: bool,
}


pub struct LatticeTimeStep {
    qubits: HashMap<Location, Qubit>,
    // qubits are at (odd, even) and (even, odd) indeces 
    processors: HashMap<Location, Processor>,
    // Bit processors are at (even, even) indeces; 
    // spin/phase processors are at (odd,odd) indeces
    age: u32, // current time (age)
    size: u32, 
}

impl LatticeTimeStep {
    fn new(input_size: u32) {
        age = 0;
        size = input_size;
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

    fn compute_syndrome<ProcessorType>(&self, processor: Processor) -> bool { 
        // syndrome is the sum of the values of the respective values of the four surrounding
        // qubits 
        let syndrome: u32 = [Cardinal::North, Cardinal::East, Cardinal::South, Cardinal::West]
            .iter()
            .map(|direction| match ProcessorType {
                ProcessorType::Bit => self.get_adjacent_qubit(direction, processor).bit as u32,
                ProcessorType::Spin => self.get_adjacent_qubit(direction, processor).spin as u32
            })
            .sum();
        return syndrome % 2 as bool;
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
            Direction::Center => return processor.clone(), 
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

    // TODO: this would be a cleaner way to implement the rules, 
    // though it may not work for the non-abelian rules, 
    // and as such it is not completely
    // implemented
    //
    // fn implement_local_rules(
    //     &self, 
    //     new_lattice: LatticeTimeStep,
    //     directions: vec<Direction>,
    //     conditions: vec<bool>, 
    //     actions: vec<Action>
    // ) -> Result<(), Error> {
    //     directions
    //         .zip(conditions)
    //         .zip(actions)
    //         .iter()
    //         .map(|(direction, condition, action)| {
    //             if self.get_adjacent_processor(direction).syndrome == condition
    //             // match direction {
    //             //     Direction::Center => if !self.get_adjacent_processor(direction)
    //             // }
    //             // if self.get_adjacent_processor(direction).syndrome {
    //             //     self.flip_syndrome()
    //             // }
    //         });
    // }

    fn local_rules(&self) -> Result<LatticeTimeStep, Error> { // TODO: implement error
        let mut new_lattice: LatticeTimeStep = self.clone();
        // iterate over all processors in self 
        // do the local rules and do the flips (flip newighboring qubit) in new_lattice 
        // this needs to be repeated for both spin and bit processors 
        // "each qubit is only ever controlled by a single processor"
        
        // for each processor (of a type)
        for (location, processor) in self.processors.into_iter() {
            // processors in the respective directions 
            let proc_n: Processor = self.get_adjacent_processor(Direction::North);
            let proc_ne: Processor = self.get_adjacent_processor(Direction::NorthEast);
            let proc_e: Processor = self.get_adjacent_processor(Direction::East);
            let proc_se: Processor = self.get_adjacent_processor(Direction::SouthEast);
            let proc_s: Processor = self.get_adjacent_processor(Direction::South);
            let proc_sw: Processor = self.get_adjacent_processor(Direction::SouthWest);
            let proc_w: Processor = self.get_adjacent_processor(Direction::West);
            let proc_nw: Processor = self.get_adjacent_processor(Direction::NorthWest);

            if location.x % Q == 0 { // W border
                if !processor.syndrome {continue} // do nothing for this processor
                else if proc_nw.syndrome {new_lattice.flip_syndrome(proc_w); continue} 
                else if proc_w.syndrome {new_lattice.flip_syndrome(proc_w); continue} 
                else if proc_sw.syndrome {new_lattice.flip_syndrome(proc_w); continue}
            }
            else if location.y % Q == 0 { // S border
                if !processor.syndrome {continue}
                else if proc_sw.syndrome {new_lattice.flip_syndrome(proc_s); continue}
                else if proc_s.syndrome {new_lattice.flip_syndrome(proc_s); continue}
                else if proc_se.syndrome {new_lattice.flip_syndrome(proc_s); continue}
            }
            else {
                if location.x % Q < floor(Q/2) && location.y % Q < floor(Q/2) { // SW quadrant
                 if !processor.syndrome {continue}
                    else if proc_s.syndrome {continue}
                    else if proc_w.syndrome {continue}
                    else if proc_n.syndrome {new_lattice.flip_syndrome(proc_n); continue}
                    else if proc_e.syndrome {new_lattice.flip_syndrome(proc_e); continue}
                    else if proc_sw.syndrome { continue}
                    else if proc_nw.syndrome {new_lattice.flip_syndrome(proc_n); continue}
                    else if proc_se.syndrome {new_lattice.flip_syndrome(proc_e); continue}
                    else {new_lattice.flip_syndrome(proc_n); continue} // flip north or east
                }
                if location.x % Q < floor(Q/2) && location.y == Q < floor(Q/2) { // W corridor
                    if !processor.syndrome {continue}
                    else if proc_s.syndrome {continue}
                    else if proc_w.syndrome {continue}
                    else if proc_n.syndrome {continue}
                    else if proc_e.syndrome {new_lattice.flip_syndrome(proc_e); continue}
                    else if proc_sw.syndrome { continue}
                    else if proc_nw.syndrome { continue}
                    else {new_lattice.flip_syndrome(proc_e); continue} // flip east
                }
                if location.x % Q < floor(Q/2) && location.y % Q > floor(Q/2) { // NW quadrant
                    if !processor.syndrome {continue}
                    else if proc_w.syndrome {continue}
                    else if proc_n.syndrome {continue}
                    else if proc_e.syndrome {new_lattice.flip_syndrome(proc_e); continue}
                    else if proc_s.syndrome {new_lattice.flip_syndrome(proc_s); continue}
                    else if proc_nw.syndrome {continue}
                    else if proc_ne.syndrome {new_lattice.flip_syndrome(proc_e); continue}
                    else if proc_sw.syndrome {new_lattice.flip_syndrome(proc_s); continue}
                    else {new_lattice.flip_syndrome(proc_e); continue} // flip east or south
                }
                if location.x % Q == floor(Q/2) && location.y % Q > floor(Q/2) { // N corridor
                    if !processor.syndrome {continue}
                    else if proc_w.syndrome {continue}
                    else if proc_n.syndrome {continue}
                    else if proc_e.syndrome {continue}
                    else if proc_s.syndrome {new_lattice.flip_syndrome(proc_s); continue}
                    else if proc_nw.syndrome {continue}
                    else if proc_ne.syndrome {continue}
                    else {new_lattice.flip_syndrome(proc_s); continue} // flip south
                }
                if location.x % Q > floor(Q/2) && location.y % Q > floor(Q/2) { // NE quadrant
                    if !processor.syndrome {continue}
                    else if proc_n.syndrome {continue}
                    else if proc_e.syndrome {continue}
                    else if proc_s.syndrome {new_lattice.flip_syndrome(proc_s); continue}
                    else if proc_w.syndrome {new_lattice.flip_syndrome(proc_w); continue}
                    else if proc_ne.syndrome {continue}
                    else if proc_se.syndrome {new_lattice.flip_syndrome(proc_s); continue}
                    else if proc_nw.syndrome {new_lattice.flip_syndrome(proc_w); continue}
                    else {new_lattice.flip_syndrome(proc_w); continue} // flip west or south
                }
                if location.x % Q > floor(Q/2) && location.y % Q == floor(Q/2) { // E corridor
                    if !processor.syndrome {continue}
                    else if proc_n.syndrome {continue}
                    else if proc_e.syndrome {continue}
                    else if proc_s.syndrome {continue}
                    else if proc_w.syndrome {new_lattice.flip_syndrome(proc_w); continue}
                    else if proc_ne.syndrome {continue}
                    else if proc_se.syndrome {continue}
                    else {new_lattice.flip_syndrome(proc_w); continue} // flip west
                }
                if location.x % Q > floor(Q/2) && location.y % Q < floor(Q/2) { // SE quadrant
                    if !processor.syndrome {continue}
                    else if proc_e.syndrome {continue}
                    else if proc_s.syndrome {continue}
                    else if proc_w.syndrome {new_lattice.flip_syndrome(proc_w); continue}
                    else if proc_n.syndrome {new_lattice.flip_syndrome(proc_n); continue}
                    else if proc_se.syndrome {continue}
                    else if proc_sw.syndrome {new_lattice.flip_syndrome(proc_w); continue}
                    else if proc_ne.syndrome {new_lattice.flip_syndrome(proc_n); continue}
                    else {new_lattice.flip_syndrome(proc_w); continue} // flip west or north
                }
                if location.x % Q == floor(Q/2) && location.y % Q < floor(Q/2) { // S corridor
                    if !processor.syndrome {continue}
                    else if proc_e.syndrome {continue}
                    else if proc_s.syndrome {continue}
                    else if proc_w.syndrome {continue}
                    else if proc_n.syndrome {new_lattice.flip_syndrome(proc_n); continue}
                    else if proc_se.syndrome {continue}
                    else if proc_sw.syndrome {continue}
                    else {new_lattice.flip_syndrome(proc_n); continue} // flip north
                }
                if location.x % Q == floor(Q/2) && location.y % Q == floor(Q/2) { // colony center
                    continue;
                    // if age == 0, execute rules for colony neighbors and update flipsignal
                    // else, do nothing
                }
            }
        }
    }

    fn step(&self) {
        age += 1 % U;

        self = self.local_rules();
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

        let mut new_step: LatticeTimeStep = steps[steps.length-1];
        vec.push(new_step.step()) //add the next lattice time step


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
