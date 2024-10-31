mod anyon;
mod braiding;
mod fusion;
mod gates;
mod state;
mod util;
use pyo3::prelude::*;

/// This builds the bindings for maturin and enables the python module to be
/// imported. For any new class which should be accessible by python, add it
/// here following the same format
#[pymodule]
fn tqc_emu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<model::anyon::Anyon>()?;
    // m.add_class::<model::anyon::IsingTopoCharge>()?;
    // m.add_class::<model::anyon::FibonacciTopoCharge>()?;
    // m.add_class::<model::anyon::TopoCharge>()?;

    // m.add_class::<model::model::Model>()?;
    // m.add_class::<model::model::AnyonModel>()?;

    // m.add_class::<fusion::fusion::Fusion>()?;
    // m.add_class::<fusion::fusion::FusionPair>()?;

    // m.add_class::<fusion::state::State>()?;

    // m.add_class::<util::basis::Basis>()?;
    // m.add_class::<util::statevec::StateVec>()?;
    Ok(())
}
