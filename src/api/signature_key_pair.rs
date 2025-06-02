use pyo3::prelude::{*};
use pyo3::exceptions::PyValueError;
use openmls::prelude::{SignatureScheme,OpenMlsProvider};
use openmls_basic_credential::SignatureKeyPair;
use openmls_traits::signatures::Signer;

use super::signature_scheme::PySignatureScheme;
use super::openmls_rust_crypto_provider::PyOpenMlsRustCrypto;

#[derive(Debug)]
#[pyclass(name="SignatureKeyPair")]
pub struct PySignatureKeyPair {
    pub wrapped : SignatureKeyPair,
    pub signature_scheme : SignatureScheme,

}

#[pymethods]
impl PySignatureKeyPair {

    #[new]
    pub fn new(py_signature_scheme: PySignatureScheme) -> PyResult<Self> {
        let sig_scheme_code = py_signature_scheme.value();
        if let Ok(signature_scheme) = SignatureScheme::try_from(sig_scheme_code) {
            Ok(Self {
                    // Currently only supports ECDSA_SECP256R1_SHA256 and ED25519
                    wrapped: SignatureKeyPair::new(signature_scheme).expect("Could not generate signature key pair"),
                    signature_scheme: signature_scheme,
                })
        } else {        
            let error_msg = format!("Could not generate signature scheme for code: {:?}",sig_scheme_code);
            Err(PyValueError::new_err(error_msg))
        }
    }

    // pub fn store(&self, storage_provider: &PyStorageProvider) {
    //     self.wrapped.store(storage_provider.wrapped);
    // }
    pub fn store_in_provider(&self, provider: &PyOpenMlsRustCrypto) {
        self.wrapped.store(provider.wrapped.storage()).expect("It must be possible to store keys.");
    }

    pub fn public(&self) -> PyResult<&[u8]> {
        Ok(self.wrapped.public())
    }

    pub fn __repr__(&self) -> String {
        let pub_key = self.public().expect("Signature key pair's public key to be visible. ");
        if let Ok(sig) = self.wrapped.sign(b"\0") {
            format!("SignatureKeyPair< Public Key: {:?}, Signed null byte: {:?} >",pub_key, sig)
        } else {
            format!("SignatureKeyPair< Public Key: {:?},Error eigning null byte >",pub_key)
        }
    }
}

