use pyo3::prelude::*;
use std::collections::HashSet;
use crate::state::FusionNode;

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
/// The basis is represented as a vector of tuples (time, FusionPair). In TQC,
/// the basis is a sequence of fusion operations that occur in the fusion tree,
/// and a different fusion ordering is a different basis.
pub struct Basis {
    ops: Vec<FusionNode>,
}

#[pymethods]
impl Basis {
    #[new]
    pub fn new(ops: Vec<FusionNode>) -> Self {
        Basis { ops }
    }

    /// Verifies the basis
    /// Preconditions: FusionEvents are ordered by non-decreasing time
    pub fn verify_basis(&self, anyons: usize) -> bool {

        // Total number of fusions always is one less than the number of anyons
        if self.ops.len() != &anyons - 1 {
            false;
        }

        // Each time represents a level in the fusion tree
        let mut current_time: u32 = 0;
        let mut fused_anyons: HashSet<usize> = HashSet::new();

        for (t, fusion_event) in &self.ops {
            let anyon_1: usize = fusion_event.anyon_1();
            let anyon_2: usize = fusion_event.anyon_2();

            // Increment the time without regard for the time step
            if *t > current_time {
                current_time = *t;
            }

            // Invalid range
            // Either anyon is greater than anyons (because they are usize)
            if !(anyon_1 < anyon_2 && anyon_2 < anyons) {
                false;
            }

            // Either anyon has already been fused
            // Doesn't matter if they are in the same time step or not
            if fused_anyons.contains(&anyon_1) || fused_anyons.contains(&anyon_2){
                false;
            }

            // Checks for adjacency of anyon_1 and anyon_2 (whether there are any unfused anyons between them)
            for anyon in anyon_1..anyon_2 {
                if !fused_anyons.contains(&anyon) {
                    false;
                }
            }

            // Anyons are inserted after they have been fused because the time of
            // each FusionNode >= the previous FusionNode (time is not decreasing)
            fused_anyons.insert(anyon_1);
            fused_anyons.insert(anyon_2);
        }

        true
    }
}
