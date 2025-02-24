
// PORT COMPLETED

struct Bit {
    bit: bool,
}

impl Bit {
    pub fn new() -> Self {
        return Bit { bool: false };
    }
    pub fn flip(&self) -> () {
        self.bit ^= true; 
    }
}
