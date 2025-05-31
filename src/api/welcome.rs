use pyo3::prelude::{*};
use openmls::prelude::{*};
// use super::signature_scheme::PySignatureScheme;


/// Wrapper for OpenMLS Welcome
#[pyclass(name = "Welcome")]
pub struct PyWelcome {
    pub wrapped: Welcome,
}

#[pymethods]
impl PyWelcome {
    // #[new]
    // pub fn new(identity: Vec<u8>) -> Self {
    //     Self {
    //         wrapped: Welcome::new(identity),
    //     }
    // }

    // #[getter]
    // fn identity(&self) -> PyResult<&[u8]> {
    //     Ok(self.wrapped.identity())
    // }
}
