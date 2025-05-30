use pyo3::prelude::{*};
use openmls::prelude::{CredentialWithKey};
use super::basic_credential::PyBasicCredential;

/// Wrapper for OpenMLS CredentialWithKey
#[pyclass(name = "CredentialWithKey")]
pub struct PyCredentialWithKey {
    wrapped: CredentialWithKey,
}

#[pymethods]
impl PyCredentialWithKey {
    #[new]
    pub fn new(py_basic_credential: PyBasicCredential, public_key: &[u8]) -> Self {
        Self {
            wrapped: CredentialWithKey {
                credential: py_basic_credential.wrapped.clone().into(),
                signature_key: public_key.into(),
            } 
        }
    }

}
