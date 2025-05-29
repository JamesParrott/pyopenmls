use pyo3::prelude::{*};
use openmls::prelude::{SignatureScheme};
use openmls_basic_credential::SignatureKeyPair;
use super::signature_scheme::PySignatureScheme;
// use super::openmls_rust_crypto::PyOpenMlsRustCrypto;

#[allow(dead_code)]
#[pyclass(name="SignatureKeyPair")]
pub struct PySignatureKeyPair {
    wrapped : SignatureKeyPair,
    signature_scheme : SignatureScheme,

}

#[pymethods]
impl PySignatureKeyPair {

    #[new]
    pub fn new(py_signature_scheme: PySignatureScheme) -> Self {
        let sig_scheme_code = py_signature_scheme.value();
        let error_msg = format!("Could not generate signature scheme for code: {:?}",sig_scheme_code);
        let signature_scheme = SignatureScheme::try_from(sig_scheme_code).expect(&error_msg);
        Self {
                // Currently only supports ECDSA_SECP256R1_SHA256 and ED25519
                wrapped: SignatureKeyPair::new(signature_scheme).expect("Could not generate signature key pair"),
                signature_scheme: signature_scheme,
            }

    }

}

