

struct Simulator {
    printMovie: bool,
    L: u32,
    Q: u32,
    k: u32,
    U: u32,
    b: u32,
    tmax: u32,
    N: u32,
    p: i64,
    q: i64,
    f_n: u32,
    f_c: u32,
    code: &ToricCode,
    automata: &&&Automaton, // may want to use something else than nested references
}

impl Simulator {
    /* pub fn new() {}

    fn linspace() -> vec<i64> {}
    fn linspace() -> vec<u32> {}

    // no parameters
    fn init() {}
    fn reset() {}
    fn simulationStep() {}

    // with parameters
    fn init() {}
    fn reset() {}
    fn simulationStep() {}
    fn simulationStep() {}
    
    pub fn denerateDecayCurve() {}
    pub fn getTmax() {}
    pub fn getN() {}
    pub fn setTmax() {}
    pub fn setN() {}
    pub fn setPrintMovie() {} */
}
