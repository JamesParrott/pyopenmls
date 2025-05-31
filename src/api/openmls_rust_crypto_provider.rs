use std::default::{Default};
use std::collections::{HashMap};
use pyo3::prelude::{*};
use openmls::prelude::{OpenMlsProvider};
use openmls_rust_crypto::{OpenMlsRustCrypto};

#[derive(Debug)]
#[pyclass(name="OpenMlsRustCrypto")]
pub struct PyOpenMlsRustCrypto {
    pub wrapped : OpenMlsRustCrypto,

}

#[pymethods]
impl PyOpenMlsRustCrypto {

    // Open MlsRustcrypto is just a simple Struct with derived Default trait. 
    // Giving new args requires wrapping RustCrypto and MemoryStorage
    // openmls_rust_crypto\src\lib.rs
    // #[derive(Default, Debug)]
    // #[cfg_attr(feature = "test-utils", derive(Clone))]
    // pub struct OpenMlsRustCrypto {
    //     crypto: RustCrypto,
    //     key_store: MemoryStorage,
    // }
    #[new]
    pub fn new() -> Self {
        Self {
            wrapped: Default::default(),
        }
    }


    #[getter]
    fn storage_values(&self) -> PyResult<HashMap<Vec<u8>, Vec<u8>>> {
        let values = self.wrapped.storage().values.read().unwrap();
        Ok(values.clone())
    }
}