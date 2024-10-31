use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// We limit our topological charge (TC) to the ising model for now.
/// Eventually we will support the fibonacci model (and maybe custom models too)
pub enum TopoCharge {
    Psi,
    Vacuum,
    Sigma,
}

#[pymethods]
impl TopoCharge {
    pub fn to_string(&self) -> String {
        match self {
            TopoCharge::Psi => "Psi".to_string(),
            TopoCharge::Vacuum => "Vacuum".to_string(),
            TopoCharge::Sigma => "Sigma".to_string(),
        }
    }
}

#[pyclass]
#[derive(Clone, Debug, PartialEq)]
/// In Topological Quantum Computing, anyons are the fundamental quasiparticles
/// which enable the computation. Anyons have an associated topological charge
/// given by the model used. This struct represents an anyon with a name,
/// charge, and position.
pub struct Anyon {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    charge: TopoCharge,
    #[pyo3(get)]
    position: (f64, f64),
}

impl Anyon {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn charge(&self) -> TopoCharge {
        self.charge.clone()
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
    }
}

#[pymethods]
impl Anyon {
    #[new]
    pub fn new(name: String, charge: TopoCharge, position: (f64, f64)) -> Self {
        Anyon {
            name,
            charge,
            position,
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Anyon: name={}, charge={}, position={:?}",
            self.name,
            self.charge.to_string(),
            self.position
        ))
    }
}
