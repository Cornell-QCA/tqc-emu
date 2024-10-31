use crate::util::math::c64;
use nalgebra::DMatrix;
use pyo3::prelude::*;

use crate::state::State;

/// A braid is a sequence of swaps that can be applied to a state. A sequence of
/// braids is analogous to a gate in a quantum circuit for TQC.
#[pyclass]
struct Braid {
    #[pyo3(get)]
    state: State,
    #[pyo3(get)]
    swaps: Vec<(usize, usize)>,
    braid_mtx: DMatrix<c64>,
}

impl Braid {
    // TODO: Write swap

    //  TODO: Write swap_to_qubit

    // TODO: Write swap_mtx

    // TODO: Write unitary

    // TODO: Write str
}

#[pymethods]
impl Braid {
    #[new]
    pub fn new(state: State) -> Self {
        if state.anyons().len() < 3 {
            panic!("State must have at least 3 anyons to braid");
        }

        Braid {
            state,
            swaps: Vec::new(),
            braid_mtx: DMatrix::from_element(1, 1, c64::new(0.0, 0.0)),
        }
    }
}
