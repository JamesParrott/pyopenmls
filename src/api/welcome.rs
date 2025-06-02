use pyo3::prelude::{*};
use openmls::prelude::{*};
// use super::signature_scheme::PySignatureScheme;


/// Wrapper for OpenMLS Welcome
#[derive(Debug,Clone)]
#[pyclass(name = "Welcome")]
pub struct PyWelcome {
    pub wrapped: Welcome,
}


