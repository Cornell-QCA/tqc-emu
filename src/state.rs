use crate::{anyon::Anyon, fusion::fusion::FusionEvent, util::statevec::StateVec};
use pyo3::prelude::*;

/// In the fusion tree, each node is a tuple with an associated time and fusion
/// event. We use this type to represent the elements in the fusion tree.
pub type FusionNode = (u32, FusionEvent);

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
