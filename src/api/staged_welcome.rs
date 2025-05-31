use pyo3::prelude::{*};
use openmls::prelude::{*};
// use super::signature_scheme::PySignatureScheme;


/// Wrapper for OpenMLS StagedWelcome
#[pyclass(name = "StagedWelcome")]
pub struct PyStagedWelcome {
    pub wrapped: StagedWelcome,
}

#[pymethods]
impl PyStagedWelcome {
    // #[new]
    // pub fn new(identity: Vec<u8>) -> Self {
    //     Self {
    //         wrapped: StagedWelcome::new(identity),
    //     }
    // }

    // #[getter]
    // fn identity(&self) -> PyResult<&[u8]> {
    //     Ok(self.wrapped.identity())
    // }
}
