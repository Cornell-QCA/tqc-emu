use super::anyon::Anyon;
use crate::fusion::fusion::FusionPair;
use pyo3::prelude::*;

use crate::util::math::c64;
use nalgebra::{base::DVector, DMatrix};
use numpy::{PyArray1, PyReadonlyArray1, ToPyArray};

/// In the fusion tree, each node is a tuple with an associated time and fusion
/// event. We use this type to represent the elements in the fusion tree.
pub type FusionNode = (u32, FusionPair);

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
/// State Vector for the system
pub struct StateVec {
    vec: DVector<c64>,
    #[pyo3(get)]
    init_size: usize,
}

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

impl StateVec {
    /// Returns a clone of the state vector
    pub fn get_vec(&self) -> DVector<c64> {
        self.vec.clone()
    }

    pub fn apply_op(&mut self, op: DMatrix<c64>) {
        self.vec = op * self.vec.clone();
    }

    /// Modifies the norm of the state vector to 1
    pub fn normalize(&mut self) {
        let norm = self.vec.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
        for i in 0..self.vec.len() {
            self.vec[i] /= c64::new(norm, 0.0);
        }
    }
}

impl State {
    pub fn anyons(&self) -> Vec<Anyon> {
        self.anyons.clone()
    }

    pub fn fusion_ops(&self) -> Vec<FusionNode> {
        self.fusion_ops.clone()
    }
}

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

#[pymethods]
impl StateVec {
    #[new]
    /// Creates a new state vector. If no vector is provided, it will be
    /// initialized to |0> for all qubits. Additionally, the vector will be
    /// normalized.
    pub fn new(qubit_num: usize, vec: Option<PyReadonlyArray1<c64>>) -> Self {
        let init_size = 2 << (qubit_num - 1);
        let vec = match vec {
            Some(vec) => {
                let vec = vec.as_array().to_owned();
                let mut state_vec = DVector::zeros(init_size);
                for (i, val) in vec.iter().enumerate() {
                    state_vec[i] = *val;
                }
                state_vec
            }
            None => DVector::from_iterator(
                init_size,
                (0..init_size).map(|x| {
                    if x % 2 == 0 {
                        c64::new(1.0, 0.0)
                    } else {
                        c64::new(0.0, 0.0)
                    }
                }),
            ),
        };

        // normalize the vector
        let mut state_vec = StateVec { vec, init_size };
        state_vec.normalize();
        state_vec
    }

    #[getter]
    fn vec(&self, py: Python<'_>) -> PyResult<Py<PyArray1<c64>>> {
        let vec = self.vec.iter().map(|x| *x).collect::<Vec<c64>>();
        Ok(vec.to_pyarray(py).to_owned())
    }

    #[setter]
    fn set_vec(&mut self, vec: PyReadonlyArray1<c64>) {
        let vec = vec.as_array().to_owned();
        for (i, val) in vec.iter().enumerate() {
            self.vec[i] = *val;
        }
    }

    #[setter]
    pub fn set_size(&mut self, qubit_num: usize) {
        self.init_size = 2 << qubit_num;
        self.vec = DVector::zeros(self.init_size);
    }

    pub fn __str__(&self) -> PyResult<String> {
        let mut output: String = "[\n".to_string();
        for val in self.vec.iter() {
            output.push_str(&format!("\t{:?} + {:?}i\n", val.re, val.im));
        }
        output.push_str("]");
        Ok(output)
    }
}
