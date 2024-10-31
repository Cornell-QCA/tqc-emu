use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// We represent an anyon's topological charge as a triple of usizes. The values
/// serve as the combinatoric labels for the various states.
/// <br>We define the following:<br>
/// Psi: [1,0,0]<br>
/// Vacuum: [0,1,0]<br>
/// Sigma: [0,0,1]
type CanonicalTC = [usize; 3];

#[pyclass]
#[derive(Clone, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct FusionEvent {
    #[pyo3(get)]
    /// The first anyon index used in the fusion event
    anyon_1: usize,
    #[pyo3(get)]
    /// The first anyon index used in the fusion event
    anyon_2: usize,
}

#[pyclass]
/// A fusion is a sequence of fusion events in a fusion tree.
/// Fusion is analogous to measurement in TQC, with different orderings
/// corresponding to different bases.
pub struct Fusion {
    #[pyo3(get)]
    events: Vec<FusionEvent>,
}

impl FusionEvent {
    pub fn anyon_1(&self) -> usize {
        self.anyon_1
    }
    pub fn anyon_2(&self) -> usize {
        self.anyon_2
    }
}

impl Fusion {

}

#[pymethods]
impl FusionEvent {
    #[new]
    fn new(anyon_1: usize, anyon_2: usize) -> Self {
        FusionEvent { anyon_1, anyon_2 }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("({} {})", self.anyon_1, self.anyon_2))
    }
}

#[pymethods]
impl Fusion {

}
