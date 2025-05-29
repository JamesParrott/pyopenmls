use pyo3::prelude::{*};
use openmls::prelude::{*};
use strum_macros::{Display, FromRepr};


#[allow(non_camel_case_types)]
#[pyclass(eq, eq_int, name="SignatureScheme")]
#[derive(Display, FromRepr, Debug, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum PySignatureScheme {
    ECDSA_SECP256R1_SHA256 = SignatureScheme::ECDSA_SECP256R1_SHA256 as u16,
    ECDSA_SECP384R1_SHA384 = SignatureScheme::ECDSA_SECP384R1_SHA384 as u16,
    ECDSA_SECP521R1_SHA512 = SignatureScheme::ECDSA_SECP521R1_SHA512 as u16,
    ED25519 = SignatureScheme::ED25519 as u16,
    ED448 = SignatureScheme::ED448 as u16,
}

#[pymethods]
impl PySignatureScheme {

    pub fn name(&self) -> String {
        self.to_string()
    }
    pub fn value(&self) -> u16 {
        *self as u16
    }


}