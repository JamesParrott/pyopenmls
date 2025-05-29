use pyo3::prelude::{*};
use openmls::{prelude::{*,  tls_codec::*}};
use strum_macros::FromRepr;


#[allow(non_camel_case_types)]
#[pyclass(eq, eq_int, name="SignatureScheme")]
#[derive(FromRepr, Debug, PartialEq)]
#[repr(u16)]
pub enum PySignatureScheme {
    ECDSA_SECP256R1_SHA256 = SignatureScheme::ECDSA_SECP256R1_SHA256 as u16,
    ECDSA_SECP384R1_SHA384 = SignatureScheme::ECDSA_SECP384R1_SHA384 as u16,
    ECDSA_SECP521R1_SHA512 = SignatureScheme::ECDSA_SECP521R1_SHA512 as u16,
    ED25519 = SignatureScheme::ED25519 as u16,
    ED448 = SignatureScheme::ED448 as u16,
}

// #[pymethods]
// impl PyCiphersuite {
//     // #[new]
//     // fn new(value: i32) -> Self {
//     //     Number(value)
//     // }
//     pub const fn signature_algorithm(&self) -> PyResult<PySignatureScheme> {
//         if Ok(cipher_suite) = Ciphersuite.try_from(self as isize) {
//             let signature_scheme = cipher_suite.signature_algorithm();
            
//         } else {

//         }
//     }
// }