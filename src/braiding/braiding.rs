use crate::util::math::c64;
use nalgebra::DMatrix;
use pyo3::prelude::*;
use std::collections::HashSet;

use crate::util::state::State;
use crate::error::Error;

/// A braid is a sequence of swaps that can be applied to a state. A sequence of
/// braids is analogous to a gate in a quantum circuit for TQC.
#[pyclass]
pub struct Braid {
    #[pyo3(get)]
    state: State,
    #[pyo3(get)]
    swaps: Vec<Vec<(usize, usize)>>,
    braid_mtx: DMatrix<c64>,
}

impl Braid {
    /// Swaps the position of anyons in `state.anyons` for each specified swap in `swaps`.
    /// Swaps can only be performed on adjacent anyons, and each anyon can only be swapped once per `swap` call.
    pub fn swap(&mut self, swaps: Vec<(usize, usize)>) -> Result<(), Error> {
        let mut applied_swaps: Vec<(usize, usize)> = Vec::new();
        let mut swapped_indices = HashSet::new();

        for (index_a, index_b) in swaps {
            if (index_a as isize - index_b as isize).abs() != 1 {
                return Error::BraidingError(format!("Indices {} and {} are not adjacent", index_a, index_b));
            }

            if swapped_indices.contains(&index_a) {
                return Error::BraidingError(format!("Index {} has already been swapped in this operation", index_a));
            }

            if swapped_indices.contains(&index_b) {
                return Error::BraidingError(format!("Index {} has already been swapped in this operation", index_b));
            }

            self.state.swap_anyons(index_a, index_b).map_err(|e| Error::BraidingError(format!("Failed to swap anyons: {}", e)));

            applied_swaps.push((index_a, index_b));
            swapped_indices.insert(index_a);
            swapped_indices.insert(index_b);
        }

        self.swaps.push(applied_swaps);
        Ok(())
    }

    // TODO: Write swap_to_qubit

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
