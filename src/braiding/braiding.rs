use crate::fusion::fusion::Fusion;
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
    fusion: Fusion,
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

    /// Determines which qubit a specified swap operation is acting on.
    pub fn swap_to_qubit(&self, time: usize, swap_index: usize) -> Result<Option<usize>, Error> {
        if time == 0 || time > self.swaps.len() {
            return Error::BraidingError(
                format!(
                    "Time cannot be 0 or greater than the length of the swaps. Inputted time: {}, swaps length: {}",
                    time, 
                    self.swaps.len()
                )
            );
        }

        let swap = &self.swaps[time - 1];
        if swap_index >= swap.len() {
            return Error::BraidingError(
                format!(
                    "Swap index {} is greater than the number of swaps {}",
                    swap_index,
                    swap.len(),
                )
            );
        }

        let (index_a, index_b) = swap[swap_index];

        Ok(self.fusion.qubit_enc().iter().enumerate().find_map(|(qubit_index, fusion_pair)| {
            if (index_a == fusion_pair.anyon_1() && index_b == fusion_pair.anyon_2()) ||
                (index_a == fusion_pair.anyon_2() && index_b == fusion_pair.anyon_1()) {
                    Some(qubit_index)
            } else {
                None
            }  
        }))
    }

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
            fusion: Fusion(state),
            braid_mtx: DMatrix::from_element(1, 1, c64::new(0.0, 0.0)),
        }
    }
}
