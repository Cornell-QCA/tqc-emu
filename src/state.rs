use crate::{anyon::Anyon, fusion::fusion::FusionPair, util::statevec::StateVec};
use pyo3::prelude::*;

/// In the fusion tree, each node is a tuple with an associated time and fusion
/// event. We use this type to represent the elements in the fusion tree.
pub type FusionNode = (u32, FusionPair);

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
/// State is the overall state of our system. It stores everything to fully
/// describe an anyon system and its associated operations.
pub struct State {
    #[pyo3(get)]
    anyons: Vec<Anyon>,
    #[pyo3(get)]
    fusion_ops: Vec<FusionNode>,
    // TODO: Add braiding
    // #[pyo3(get)]
    // braiding_ops: Vec<FusionNode>,
    #[pyo3(get)]
    state_vec: StateVec,
}

impl State {
    pub fn anyons(&self) -> Vec<Anyon> {
        self.anyons.clone()
    }

    pub fn fusion_ops(&self) -> Vec<FusionNode> {
        self.fusion_ops.clone()
    }
}


/// Python Methods
#[pymethods]
impl State {
    #[new]
    fn new() -> Self {
        State {
            anyons: Vec::new(),
            fusion_ops: Vec::new(),
            state_vec: StateVec::new(1, None),
        }
    }

    /// Add an anyon to the state
    fn add_anyon(&mut self, anyon: Anyon) -> PyResult<bool> {
        self.anyons.push(anyon);
        Ok(true)
    }

    // TODO: Add braiding

    //  TODO: Create a method for adding an operation (and verifying it with an internal method)
}
