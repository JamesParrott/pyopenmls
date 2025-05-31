use pyo3::prelude::{*};
use openmls::prelude::{*};


/// Wrapper for OpenMLS RatchetTreeIn
#[pyclass(name = "RatchetTreeIn")]
pub struct PyRatchetTreeIn {
    pub wrapped: RatchetTreeIn,
}

// #[pymethods]
// impl PyRatchetTreeIn {

// }