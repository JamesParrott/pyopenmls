use pyo3::prelude::{*};
use openmls::prelude::{*};

/// Wrapper for OpenMLS BasicCredential
#[pyclass(name = "BasicCredential")]
pub struct PyBasicCredential {
    wrapped: BasicCredential,
}

#[pymethods]
impl PyBasicCredential {
    #[new]
    pub fn new(identity: Vec<u8>) -> Self {
        Self {
            wrapped: BasicCredential::new(identity),
        }
    }

    #[getter]
    fn identity(&self) -> PyResult<&[u8]> {
        Ok(self.wrapped.identity())
    }
}