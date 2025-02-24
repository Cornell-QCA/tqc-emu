
// TODO: modules
//       consider structures other than nested references

struct Automaton {
    hierarchyDepth: u32,
    syndromes: [bool; 9],
    f_c: i64,
    f_n: i64,
    printMovie: bool,
    memory: vec<&AutomatonMemoryDataset>,
    neighbors: &&Automaton, 
    qubits: &&Qubit,
}

impl Automaton {
    // TODO: implement
    /* fn new() -> Self {}


    fn localUpdateRules() -> u32 {}
    fn ipow() -> u32 {}
    fn printRule() {}

    pub fn setNeighbors() {}
    pub fn setQubits() {}
    pub fn setSyndrome() {}
    pub fn localDependencies() {}
    pub fn executeFlips() {}
    pub fn caUpdates() {} */
}
