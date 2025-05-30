use std::default::{Default};

use pyo3::prelude::{*};
use openmls_rust_crypto::{OpenMlsRustCrypto};
// use super::openmls_rust_crypto_provider::PyStorageProvider;

#[allow(dead_code)]
#[pyclass(name="OpenMlsRustCrypto")]
pub struct PyOpenMlsRustCrypto {
    wrapped : OpenMlsRustCrypto,

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

    // pub fn storage(&self) -> PyResult<PyStorageProvider> {
        // OK(PyStorageProvider(self.wrapped))
    // }

}