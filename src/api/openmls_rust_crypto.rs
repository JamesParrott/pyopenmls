use std::default::{Default};

use pyo3::prelude::{*};
use openmls_rust_crypto::{OpenMlsRustCrypto};

#[allow(dead_code)]
#[pyclass(name="OpenMlsRustCrypto")]
pub struct PyOpenMlsRustCrypto {
    wrapped : OpenMlsRustCrypto,

}

#[pymethods]
impl PyOpenMlsRustCrypto {

    #[new]
    pub fn new() -> Self {
        Self {
            wrapped: Default::default(),
        }
    }

}