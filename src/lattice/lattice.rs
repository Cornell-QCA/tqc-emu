use std::collections::HashMap;
use math::round::floor;

enum ProcessorType {
    Bit, // primary lattice
    Spin, // dual lattice
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

// TODO: may need to change the encoding
enum LogicalQubitType {
    Vertical,
    Horizontal,
}

// regards the implement_flip_syndrome optimization
// enum Action {
//     DoNothing,
//     FlipNorth,
//     FlipEast,
//     FlipSouth,
//     FlipWest,
// }
pub struct TotalColony {
// all colonies and supercolonies (consisting of (NxN)^n lattices)
}

pub struct Lattice { // NOTE: the lattice is now incremented by integers to not have floating point
    // rounding errors
    colony_size: usize, // size (NxN) of a colony, scales exponentially with colony_levels
    colony_levels: usize, // total amount of levels of super colonies
    work_time: usize, // perfect square, equal to n times n: n is the amount of intervals in a work period and size of each work period interval
    total_work_periods: usize, // amount of work periods we want to run
    f_c: usize, // error threshold for the center cell of a colony
    f_n: usize, // error threshold for a neighboring cell (not center)
    
    steps: Vec<LatticeTimeStep>, // Vector of ToricCodeTimeStep's, each representing the state of the toric code at a point in time
    intervals: Vec<Interval>, //intervals containing sqrt(work_time) amount of steps
    work_periods: Vec<WorkPeriod>, //work periods containing sqrt(work_time) amount of intervals
}

pub struct Interval {

}

pub struct WorkPeriod{

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

impl TotalColony {


}


impl Lattice {
    pub fn new(size: usize, levels: usize, time: usize, periods: usize, f1: usize, f2: usize) -> Self {
        Lattice {
            colony_size: size,
            colony_levels: levels,
            work_time: time,
            total_work_periods: periods,
            f_c: f1,
            f_n: f2,
            steps: Vec::new(),
            intervals: Vec::new(),
            work_periods: Vec::new(),
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


impl Location {
    fn new(x: u32, y:u32) -> Location {
        return Location { x, y };
    }

    // fn get_location_type(&self) -> 
}

// TODO: put each processor and qubit into the respective hashmap
//       add return type for `new`
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
            // X operator
            Bit => &cardinals 
                .iter()
                .map(|direction| self.get_adjacent_qubit(direction, processor).bit ^= true),
            // Z operator
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
    /* though it may not work for the non-abelian rules, 
    and as such it is not completely
    implemented

    fn implement_local_rules(
        &self, 
        new_lattice: LatticeTimeStep,
        directions: vec<Direction>,
        conditions: vec<bool>, 
        actions: vec<Action>
    ) -> Result<(), Error> {
        directions
            .zip(conditions)
            .zip(actions)
            .iter()
            .map(|(direction, condition, action)| {
                if self.get_adjacent_processor(direction).syndrome == condition
                // match direction {
                //     Direction::Center => if !self.get_adjacent_processor(direction)
                // }
                // if self.get_adjacent_processor(direction).syndrome {
                //     self.flip_syndrome()
                // }
            });
    } */

    fn local_rules(&self) -> Result<LatticeTimeStep, Error> { // TODO: implement error handling
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

    // Adds a new logical qubit in the orientation (vertical/horizontal) representing two of the
    // base states on the torus
    // Whether the logical qubit is on the primary or dual lattice is determined by the specified
    // processor and its location
    // TODO: the encoding may be incorrect and/or may need to add the state that has two loops on
    // the torus, each of different orientation 
    // NOTE: currently this is just a function for testing the lattice until I learn exactly how the
    // logical qubits are encoded
    fn add_logical_qubit<LogicalQubitType>(&self, processor: Processor) -> Result<(), Error> {
        // add a new loop on the torus
        match LogicalQubitType {
            LogicalQubitType::Vertical => { 
                for y in 0..self.size {
                    let changed_processor: Processor = self.processors.get(&processor.x, (&processor.y + y) % self.size);
                    self.flip_syndrome(changed_processor);
                } 
            },
            LogicalQubitType::Horizontal => {
                for x in 0..self.size {
                    let changed_processor: Processor = self.processors.get((&processor.x + x) % self.size, &processor.y);
                    self.flip_syndrome(changed_processor);
                } 
            },
        }
    }

    fn step(&self) {  
        age += 1 % U;

        self = self.local_rules();
    }
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

        let mut new_step: LatticeTimeStep = steps[steps.length-1]; // TODO: does rust have indexing
    // from the end?
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
