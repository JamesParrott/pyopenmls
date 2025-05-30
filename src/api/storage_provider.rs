use std::default::{Default};

use pyo3::prelude::{*};
use openmls_rust_crypto::MemoryStorage;

#[allow(dead_code)]
#[pyclass(name="StorageProvider")]
pub struct PyStorageProvider {
    wrapped : MemoryStorage,

}

#[pymethods]
impl PyStorageProvider {


    #[new]
    pub fn new() -> Self {
        Self {
            wrapped: Default::default(),
        }
    }


}