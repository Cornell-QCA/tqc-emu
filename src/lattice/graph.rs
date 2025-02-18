// For developing decoders on more general topologies than tori by using graphs to determine
// neighbors in a more flexible manner than using 2d vectors.

// A trait that all cellulations/graphs will implement
trait Cellulation {
    // A cellulation is pretty much the general structure where the qubits are edges and the
    // processors/ancilli are nodes (I think).
    // Eg for the toric code, the plaquettes look like cells.
    // TODO: I think it would be good for this code to be as modular/extensible as is reasonable
    // TODO: perhaps use a hashmap for cell/qubit/other element id's
}

trait Decoder {
    // A decoder is effectively a set of rules that transform the system to mitigate error (eg
    // harrington's rules)
    // Other types of decoders may include floquet codes, color codes, MWPM (minimum weight perfect
    // matching) (MWPM already has a crate I think), or even neural nets (see google's alphaqubit
    // decoder for surface codes)
}

struct ToricCodeWithDefects {
    // TODO: do we want hashmaps or some dedicated graph?
    // Perhaps we could still use a lattice for this one, though it would be more difficult for
    // later topologies (eg the genus 5 one)
}

struct HarringtonDecoder {

}

impl Decoder for HarringtonDecoder {

}

impl Cellulation for ToricCodeWithDefects {

}
