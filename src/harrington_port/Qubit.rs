
// PORT COMPLETED

struct Qubit {
    bit: bool,
}

impl Qubit {
    pub fn new() -> Self {
        return Qubit { bool: false };
    }
    pub fn flip(&self) -> () {
        self.bit ^= true; 
    }
}
