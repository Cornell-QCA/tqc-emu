
use Bit; // TODO: there is syntax and organization with modules that needs to be done

struct Point {
    x: u32,
    y: u32,
}

struct AutomatonMemoryDataset {
    age: u32,
    countSignal: vec<u32>,
    newCountSignal: vec<u32>,
    flipSignal: vec<&Bit>,
    newFlipSignal: vec<&Bit>,
    count: vec<vec<u32>>,
    address: Point,
    Q: u32,
    U: u32,
    b: u32,
    isFlipedSignal: vec<u32>,
    newIsFlipedSignal: vec<u32>,
}

impl AutomatonMemoryDataset {
    pub fn new(address: Point, Q: u32, U: u32, b: u32) {
        // TODO: implementation
    }
}
