use pyo3::prelude::*;
use openmls::{prelude::{*,  tls_codec::*}};
use openmls_rust_crypto::{OpenMlsRustCrypto};
use openmls_basic_credential::SignatureKeyPair;

// Define ciphersuite ...
// const ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;
// // ... and the crypto provider to use.
// const provider = &OpenMlsRustCrypto::default();


/// Wrapper for OpenMLS BasicCredential
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(name = "BasicCredential")]
struct PyBasicCredential {
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
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyopenmls(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PyBasicCredential>()?;
    Ok(())
}
