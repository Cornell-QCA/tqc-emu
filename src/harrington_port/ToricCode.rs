
struct ToricCode {
    L: u32,
    vertices: &&u32,
    edges: &&&Qubit, // TODO: consider more rust-native architecture
    starSyndromes: &&u32,
    faultySyndromes: &&u32,
    p: i64,
    q: i64,
    printMovie: bool,
    
    // random libraries
}

impl ToricCode {
    /* pub fn new() -> Self {}

    fn hasLogicalError() -> bool {}
    fn flip() -> () {}
    fn dump() -> () {}
    fn getQubit() -> &Qubit {}

    pub fn reset() {}
    pub fn index() -> u32 {}
    pub fn starCheck() -> u32 {}
    pub fn flip() {}
    pub fn getStarSyndromes() -> &&u32 {}
    pub fn introduceErrors() {}
    pub fn hasSyndrome() {}
    pub fn hasLogicalError() {}
    pub fn getQubits() {}
    pub fn dump() {}
    pub fn getDistance() {}
    pub fn setQ() {}
    pub fn viewQubitErrors() {}
    pub fn viewSyndromeErrors() {}
    pub fn viewStarSyndromes() {}
    pub fn viewRealStarSyndromes() {} */
    // we are not useing minimum weight perfect matching (MWPM) which is another decoder
}
