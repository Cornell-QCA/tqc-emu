mod braiding;
mod fusion;
mod gates;
mod util;
mod error;
use pyo3::prelude::*;

/// This builds the bindings for maturin and enables the python module to be
/// imported. For any new class which should be accessible by python, add it
/// here following the same format
#[pymodule]
fn tqc_emu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<util::anyon::Anyon>()?;
    m.add_class::<util::anyon::TopoCharge>()?;
    m.add_class::<util::basis::Basis>()?;
    m.add_class::<util::state::StateVec>()?;
    m.add_class::<util::state::State>()?;

    m.add_class::<fusion::fusion::Fusion>()?;
    m.add_class::<fusion::fusion::FusionPair>()?;

    m.add_class::<braiding::braiding::Braid>()?;

    Ok(())
}
