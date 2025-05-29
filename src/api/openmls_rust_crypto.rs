#![allow(unused)]
use std::default::{Default};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::{*};
use openmls::prelude::{*};
use openmls_rust_crypto::{OpenMlsRustCrypto};

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