use nalgebra::{base::DVector, DMatrix};
use numpy::{PyArray1, PyReadonlyArray1, ToPyArray};
use crate::util::math::c64;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
/// State Vector for the system
pub struct StateVec {
    vec: DVector<c64>,
    #[pyo3(get)]
    init_size: usize,
}

/// Internal Methods
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

/// Python Methods
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
