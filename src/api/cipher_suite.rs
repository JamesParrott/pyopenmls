use pyo3::exceptions::PyValueError;
use pyo3::prelude::{*};
use openmls::prelude::{*};
use super::signature_scheme::PySignatureScheme;
use strum_macros::{Display, FromRepr};


#[allow(non_camel_case_types)]
#[pyclass(eq, eq_int, name="Ciphersuite")]
#[derive(Display, FromRepr, Debug, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum PyCiphersuite {
    MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 as u16,
    MLS_128_DHKEMP256_AES128GCM_SHA256_P256 = Ciphersuite::MLS_128_DHKEMP256_AES128GCM_SHA256_P256 as u16,
    MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 = Ciphersuite::MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519 as u16,
    MLS_256_DHKEMX448_AES256GCM_SHA512_Ed448 = Ciphersuite::MLS_256_DHKEMX448_AES256GCM_SHA512_Ed448 as u16,
    MLS_256_DHKEMP521_AES256GCM_SHA512_P521 = Ciphersuite::MLS_256_DHKEMP521_AES256GCM_SHA512_P521 as u16,
    MLS_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 = Ciphersuite::MLS_256_DHKEMX448_CHACHA20POLY1305_SHA512_Ed448 as u16,
    MLS_256_DHKEMP384_AES256GCM_SHA384_P384 = Ciphersuite::MLS_256_DHKEMP384_AES256GCM_SHA384_P384 as u16,
    MLS_256_XWING_CHACHA20POLY1305_SHA256_Ed25519 = Ciphersuite::MLS_256_XWING_CHACHA20POLY1305_SHA256_Ed25519 as u16,
}

#[pymethods]
impl PyCiphersuite {

    pub fn name(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
    pub fn value(&self) -> PyResult<u16> {
        Ok(*self as u16)
    }

    pub fn signature_algorithm(&self) -> Result<PySignatureScheme, PyErr> {
        let value = self.value()?;
        if let Ok(cipher_suite) = Ciphersuite::try_from(value) {
            let signature_scheme = cipher_suite.signature_algorithm();
            Ok(PySignatureScheme::from_repr(signature_scheme as u16).unwrap())
        } else {
            Err(PyValueError::new_err("Error retrieving SignatureScheme. "))
        }
    }
}
