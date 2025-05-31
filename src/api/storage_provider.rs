use std::default::{Default};

use pyo3::prelude::{*};
use openmls_rust_crypto::MemoryStorage;

#[derive(Default)]
#[pyclass(name="StorageProvider")]
pub struct PyStorageProvider {
    pub wrapped : MemoryStorage,

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