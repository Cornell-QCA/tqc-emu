use pyo3::prelude::*;
use numpy::ndarray::Array1;
use numpy::{Complex64, PyArray1, PyReadonlyArray1, ToPyArray};

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
/// State Vector for the system
pub struct StateVec {
    vec: Array1<Complex64>,
    #[pyo3(get)]
    init_size: usize,
}

impl StateVec {
    /// Get a clone of the state vector without the size
    pub fn get_vec(&self) -> Array1<Complex64> {
        self.vec.clone()
    }
    /// Sets the norm of the state vector to 1 in place
    pub fn normalize(&mut self) {
        let norm = self.vec.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
        for i in 0..self.vec.len() {
            // Since the norm has no imaginary component, we divide only the real component
            self.vec[i] /= Complex64::new(norm, 0.0);
        }
    }
}

#[pymethods]
impl StateVec {
    #[new]
    /// Creates a new state vector. If no vector is provided, it will be
    /// initialized to |0> for all qubits. Additionally, the vector will be
    /// normalized.
    pub fn new(qubit_num: usize, vec: Option<PyReadonlyArray1<Complex64>>) -> Self {
        // init_size is required to create the StateVec
        let init_size = 2 << (qubit_num - 1);
        let vec = match vec {
            Some(vec) => vec.as_array().to_owned(),
            None => {
                let mut vec = vec![Complex64::new(1.0, 0.0)];
                vec.extend(vec![Complex64::new(0.0, 0.0); init_size - 1]);
                Array1::from(vec)
            }
        };

        // normalize the vector
        let mut state_vec = StateVec { vec, init_size };
        state_vec.normalize();
        state_vec
    }

    #[getter]
    pub fn vec(&self, py: Python<'_>) -> PyResult<Py<PyArray1<Complex64>>>{
        Ok(self.vec.to_pyarray_bound(py).to_owned().into())
    }

    #[setter]
    pub fn set_vec(&mut self, vec: PyReadonlyArray1<Complex64>) {
        self.vec = vec.as_array().to_owned();
    }

    #[setter]
    /// Sets size of state vector and sets all components to 0
    pub fn set_size(&mut self, qubit_num: usize) {
        self.init_size = 2 << qubit_num;
        self.vec = Array1::zeros(self.init_size);
    }

    /// Returns vector of all complex components
    // parentheses are used to denote a column vector
    pub fn __str__(&self) -> PyResult<String> {
        let mut output: String = "(\n".to_string();
        for val in self.vec.iter() {
            output.push_str(&format!("\t{:?} + {:?}i\n", val.re, val.im));
        }
        output.push_str(")");
        Ok(output)
    }
}
